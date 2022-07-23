use crate::{game::{Game, Player, Status}, field::Pos};
use libafl::bolts::rands::{Rand, RandomSeed, StdRand};

type Score = usize;

pub struct SimpleAi {
    own_player: Player,
    depth: usize,
    rng: StdRand,
}

impl SimpleAi {
    fn new(own_player: Player, depth: usize) -> Self {
        Self {
            depth,
            rng: StdRand::new(),
            own_player,
        }
    }

    fn play(&mut self, game: &Game) -> Game {
        // TODO, do something with score.
        self.play_rec(game, self.depth).0
    }

    fn play_rec(&mut self, game: &Game, depth: usize) -> (Game, Score) {
        let mut next_game: Game = game.clone();
        if depth == 0 {
            return (next_game, 0);
        }
        match game.status {
            Status::InitialMove { starting_player } => {
                let rand_val = self.rng.next() % next_game.remaining_pieces().len() as u64;
                let piece = next_game.remaining_pieces()[rand_val as usize];
                next_game.status = Status::Move {
                    next_player: starting_player.next(),
                    next_piece: piece,
                };
                // TODO
                (next_game, 0)
            }
            Status::Move {
                next_player: player,
                next_piece: piece,
            } => {
                if player == self.own_player {
                    let possible_spaces: Vec<Pos> = game.field.empty_spaces();
                    let next_player = player.next();

                    // We calculate if we will win for every possible place, that is still free and
                    // where we can place our current_piece.
                    for pos in possible_spaces {

                        // We will also have to calculate the optimal piece to give to our
                        // opponent, which is the piece with the least likely chance of winning.
                        for next_piece in game.remaining_pieces() {

                            next_game.status = Status::Move{ next_player,
                                                     next_piece: *next_piece };

                            self.play_rec(&next_game, depth-1);
                        }
                    }
                }
                // TODO
                (next_game, 0)
            }
            Status::Won { winner: _ } => (next_game, 0)
        }
    }
}
