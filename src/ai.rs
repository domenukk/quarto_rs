use crate::{
    field::Pos,
    game::{Game, Player, Status},
    piece::Piece,
    rng::RomuDuoJrRand,
};
use std::{collections::HashSet, time::Instant};

#[allow(clippy::module_name_repetitions)]
pub struct SimpleAi {
    own_player: Player,
    rng: RomuDuoJrRand,
}

impl SimpleAi {
    pub fn with_seed(own_player: Player, seed: u64) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        Self {
            rng: RomuDuoJrRand::with_seed(seed),
            own_player,
        }
    }

    /// Tries to play the game iteratively, searching for a locally optimal move
    /// Strategy:
    ///     We are given a piece by the opponent, we will then calculate all states that are
    ///     immediately reachable and check if they fulfill the winning condition.
    ///     If they do not fulfill the winning condition, we will all put them in a map here.
    ///     Then we will check the remaining pieces and find pieces that do *not* yield an
    ///     immediate win to the opponent, then for all of those pieces, we will need to
    ///     calculate all possible winning states, then try to maximize the number of winning
    ///     states that are reachable, this contributes to the "score" we will give this path.
    ///     The more winning pieces that there are, the more likely we will win?.
    #[allow(clippy::too_many_lines)]
    pub fn play_iteratively(&mut self, game: &mut Game) -> Game {
        // our theoretical game
        let t_game = game.clone();

        // Check the piece given to us, by our opponent, and get all empty spaces on the field.
        match game.status {
            // If we have the initial move, just pick a random piece.
            Status::InitialMove {
                starting_player: player,
            } => {
                assert!(self.own_player == player);
                if game.ai_reasoning {
                    println!("AI: Does not matter which piece we pick on the initial move.");
                }
                // return a random piece from `remaining_pieces`
                let random_piece = *self.rng.choose(game.remaining_pieces());
                game.initial_move(random_piece).unwrap();
                game.clone()
            }
            Status::Move {
                next_player: _,
                next_piece: our_piece,
            } => {
                // This is where the interesting stuff happens.

                let it = if game.ai_reasoning {
                    Some(Instant::now())
                } else {
                    None
                };

                // Grab the empty spaces.
                let empty_spaces = t_game.field.empty_spaces();
                if game.ai_reasoning {
                    println!(
                        "AI: There are {} empty spaces for us to put our piece on",
                        empty_spaces.len()
                    );
                }

                let mut states: Vec<(Game, Pos)> = Vec::with_capacity(16);

                for pos in empty_spaces {
                    // Construct all states.
                    let mut state = t_game.clone();
                    state
                        .field
                        .put(pos, our_piece)
                        .expect("Huh? AI should only do legal moves.");

                    // Check if any of these moves are winning.
                    if state.field.check_field_for_win() {
                        // Do early return here.
                        // next piece can be randomly chosen, as we will win this turn.
                        let mut new_game = game.clone();
                        let next_piece = if game.remaining_pieces().is_empty() {
                            our_piece
                        } else {
                            game.remaining_pieces()[0]
                        };
                        new_game
                            .do_move(pos, next_piece)
                            .expect("Ai should only do legal moves");
                        return new_game;
                    }

                    states.push((state, pos));
                }

                if game.ai_reasoning {
                    println!("AI: We have {} states for our move", states.len());
                }

                // This tracks which states we will remove after we calculate for the adversary.
                let mut removals = Vec::new();

                // This tracks the pieces, we should not pick, i.e. they allow the opponent to win.
                let mut non_picks = HashSet::new();

                // None of these states win immediately, try to check if any of the remaining
                // pieces will let the opponent win.
                for (state_idx, (state, _our_pos)) in states.iter().enumerate() {
                    // This is the piece we will give to our opponent.
                    for piece in state.remaining_pieces() {
                        // Perform all the moves our opponent could do with this piece.
                        for pos in state.field.empty_spaces() {
                            // Grab a clone
                            let mut new_state = state.clone();
                            // Perform the move
                            new_state
                                .field
                                .put(pos, *piece)
                                .expect("huh ai should only do legal moves!");

                            // Check if any of these moves are winning.
                            if new_state.field.check_field_for_win() && game.ai_reasoning {
                                println!("Piece: {piece:?} will let opponent win on pos {pos:?} if we place ours({our_piece:?}) on {pos:?}");
                                // remove these states from the states vector.
                                removals.push(state_idx);
                                // Add the piece to the non_picks.
                                non_picks.insert(piece);
                            }
                        }
                    }
                }

                let remaining_pieces = game.remaining_pieces().iter().collect::<HashSet<_>>();
                if game.ai_reasoning {
                    println!("AI: Game has {} remaining pieces", remaining_pieces.len());
                    println!(
                        "AI: We have {} pieces that we want to avoid",
                        non_picks.len()
                    );
                }

                let potential_picks: Vec<Piece> = remaining_pieces
                    .difference(&non_picks)
                    .map(|x| **x)
                    .collect();

                if game.ai_reasoning {
                    println!("AI: calculated all states that we can put things on without our opponent immediately winning after {:.4} us", it.unwrap().elapsed().as_micros());
                }

                // This means, our opponent will definitely win next round or it is a draw :(
                // Just shortcut and pick any piece.
                if potential_picks.is_empty() {
                    if game.ai_reasoning {
                        println!("AI: Loss is imminent, just give a random piece");
                    }
                    if game.remaining_pieces().is_empty() {
                        // This will be a draw.
                        game.do_move(states[0].1, our_piece).unwrap();
                        return game.clone();
                    }
                    let random_piece = *self.rng.choose(game.remaining_pieces());
                    game.do_move(states[0].1, random_piece).unwrap();
                    return game.clone();
                }
                //let potential_picks = Vec::from(potential_picks);

                let random_potential_pick = self.rng.choose(potential_picks);

                // remove the states we do not want, i.e. the next move will let our opponent win.
                let states: Vec<(Game, Pos)> = states
                    .iter()
                    .enumerate()
                    // Get all states, which do not want to remove
                    // This value is the idx.
                    .filter(|(idx, _)| !removals.contains(idx))
                    // Map to the state and pos
                    .map(|(_, state_pos)| state_pos)
                    .cloned()
                    .collect();

                // Oh no! we cannot avoid a game loss here. Just return.
                if states.is_empty() {
                    if game.ai_reasoning {
                        println!("AI: We will lose on the next move, wherever we place our piece and whichever piece we select! :<");
                    }
                    // return a random piece from `remaining_pieces`
                    let random_piece = *self.rng.choose(game.remaining_pieces());

                    let random_pos = self.rng.choose(game.field.empty_spaces());
                    game.do_move(random_pos, random_piece).unwrap();
                    return game.clone();
                }

                // Pick a random state from this list for now.
                let state = self.rng.choose(states.iter());

                // Grab the best move and then construct the new game.
                #[allow(clippy::cast_possible_truncation)]
                game.do_move(state.1, random_potential_pick)
                    .expect("ai should only do legal moves!");
                game.clone()
            }
            // On won and draw.
            _ => {
                unreachable!("Game should just terminate here.");
            }
        }
    }
}
