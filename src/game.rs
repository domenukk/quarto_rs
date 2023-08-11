use core::fmt::Display;

use crate::{
    field::{Field, Pos},
    piece::Piece,
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Player {
    PlayerOne,
    PlayerTwo,
}

impl Player {
    pub fn next(self) -> Self {
        match self {
            Self::PlayerOne => Self::PlayerTwo,
            Self::PlayerTwo => Self::PlayerOne,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::PlayerOne => f.write_str("Player 1"),
            Player::PlayerTwo => f.write_str("Player 2"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Status {
    InitialMove {
        starting_player: Player,
    },
    Move {
        next_player: Player,
        next_piece: Piece,
    },
    Won {
        winner: Player,
    },
    Draw {
        last_player: Player,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ArrayBase {
    Zero,
    One,
}

impl ArrayBase {
    #[must_use]
    #[inline]
    pub fn based(self, zero_based_val: usize) -> usize {
        if self == ArrayBase::Zero {
            zero_based_val
        } else {
            // Wrapping is a nice easter egg, and we don't panic.
            zero_based_val.wrapping_add(1)
        }
    }

    #[must_use]
    #[inline]
    pub fn unbased(self, based_val: usize) -> usize {
        if self == ArrayBase::Zero {
            based_val
        } else {
            based_val.wrapping_sub(1)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    pub array_base: ArrayBase,
    pub field: Field,
    remaining_pieces: Vec<Piece>,
    pub status: Status,
    pub ai_reasoning: bool,
    pub seed: Option<u64>,
    pub pvp: bool,
}

impl Game {
    /// Starts a new game
    pub fn new(starting_player: Player) -> Self {
        let remaining_pieces = (0..16).map(Piece::with_props).collect();

        #[allow(clippy::cast_precision_loss)]
        Self {
            array_base: ArrayBase::One,
            remaining_pieces,
            field: Field::new(),
            status: Status::InitialMove { starting_player },
            ai_reasoning: false,
            seed: None,
            pvp: false,
        }
    }

    pub fn round(&self) -> u8 {
        (((Field::SIZE * Field::SIZE - self.remaining_pieces.len()) / 2) + 1)
            .try_into()
            .unwrap()
    }

    pub fn player(&self) -> Player {
        match self.status {
            Status::InitialMove { starting_player } => starting_player,
            Status::Move { next_player, .. } => next_player,
            Status::Won { winner } => winner,
            Status::Draw { last_player } => last_player,
        }
    }

    /// Returns true if the game is running, false if it's over (`Draw` or `Won`)
    pub fn running(&self) -> bool {
        match self.status {
            Status::InitialMove { .. } | Status::Move { .. } => true,
            Status::Won { .. } | Status::Draw { .. } => false,
        }
    }

    /// Returns the winner, if the game is over, and it is not a draw
    pub fn winner(&self) -> Option<Player> {
        if let Status::Won { winner } = self.status {
            Some(winner)
        } else {
            None
        }
    }

    pub fn is_initial_move(&self) -> bool {
        matches!(self.status, Status::InitialMove { .. })
    }

    pub fn next_piece(&self) -> Option<Piece> {
        if let Status::Move { next_piece, .. } = self.status {
            Some(next_piece)
        } else {
            None
        }
    }

    pub fn pp(&self) {
        println!("Quarto, round: {}", self.round());
        println!();
        if self.running() {
            println!("{}, your move.", self.player());
        } else if let Some(winner) = self.winner() {
            println!("{winner} won!");
        } else {
            println!("Game ended in a draw!");
        }

        if !self.remaining_pieces().is_empty() {
            println!("\nRemaining Pieces:");
            self.pp_remaining_pieces();
        }
        println!("\nField:");
        self.field.pp(self.array_base);

        if let Some(piece) = self.next_piece() {
            println!("\nThe next piece to place is:");
            print!("       ");
            piece.pp();
            println!();
        }
    }

    pub fn pp_remaining_pieces(&self) {
        for (i, piece) in self.remaining_pieces().iter().enumerate() {
            if i > 0 && (i) % 3 == 0 {
                println!();
            }
            let based_i = self.array_base.based(i);
            print!("  {based_i}: ");
            if self.array_base.based(i) < 10 {
                // padding for low numbers
                print!(" ");
            }
            piece.pp();
            if i < (Field::SIZE * Field::SIZE) - 1 && (i + 1) % 3 != 0 {
                print!(",  ");
            }
        }
        println!();
    }

    /// Returns the list of remaining pieces
    pub fn remaining_pieces(&self) -> &[Piece] {
        &self.remaining_pieces
    }

    /// Gives the initial piece to the opponent, as we do not actually put a piece onto the field
    /// in the first turn.
    pub fn initial_move(&mut self, next_piece: Piece) -> Result<(), ()> {
        if let Status::InitialMove { starting_player } = self.status {
            let i = self
                .remaining_pieces()
                .iter()
                .position(|&x| x == next_piece)
                .ok_or(())?;
            self.remaining_pieces.remove(i);
            self.status = Status::Move {
                next_player: starting_player.next(),
                next_piece,
            };
            Ok(())
        } else {
            Err(())
        }
    }

    /// Next move, actually put a piece on the field, and give the next piece to the opponent or
    /// checks if a player won..
    pub fn do_move(&mut self, pos: Pos, next_piece: Piece) -> Result<(), ()> {
        // Grab the curent move that the player wants to execute
        if let Status::Move {
            next_player: player,
            next_piece: piece,
        } = self.status
        {
            // Actually perform the move on the field.
            self.field.put(pos, piece)?;

            if self.remaining_pieces().is_empty() {
                // This is a draw
                self.status = Status::Draw {
                    last_player: player,
                };
                return Ok(());
            }
            // remove the piece from `remaining_pieces`.
            let i = self
                .remaining_pieces()
                .iter()
                .position(|&x| x == next_piece)
                .ok_or(())?;
            self.remaining_pieces.remove(i);
            // Check if this piece yielded a win for this player.
            if self.field.check_field_for_win() {
                self.status = Status::Won { winner: player }
            } else {
                self.status = Status::Move {
                    next_player: player.next(),
                    next_piece,
                }
            };
            Ok(())
        } else {
            Err(())
        }
    }

    /// Undo the latest move
    #[cfg(test)]
    pub fn unmove(&mut self, last_pos: Pos) {
        let next_player = match self.status {
            Status::InitialMove { .. } => {
                panic!("Can't unmove an initial move, use unmove_initial!")
            }
            Status::Won { winner } => winner,
            Status::Draw { last_player } => last_player,
            Status::Move { next_player, .. } => next_player,
        };

        // There are only two players, so next is also prev.
        let prev_player = next_player.next();

        let last_piece = self.field.clear(last_pos).unwrap();
        self.remaining_pieces.push(last_piece);

        if self.remaining_pieces.len() == Field::SIZE * Field::SIZE {
            self.status = Status::InitialMove {
                starting_player: next_player,
            }
        } else {
            self.status = Status::Move {
                next_piece: last_piece,
                next_player: prev_player,
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::{Game, Player};

    #[test]
    fn test_move_unmove() {
        let mut game = Game::new(Player::PlayerOne);
        let post_unmove = game.field.clone();
        game.initial_move(game.remaining_pieces()[0]).unwrap();
        game.do_move((0, 0), game.remaining_pieces()[1]).unwrap();
        assert_ne!(post_unmove, game.field);
        game.unmove((0, 0));
        assert_eq!(post_unmove, game.field);
    }
}
