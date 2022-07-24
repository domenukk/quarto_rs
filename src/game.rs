use libafl::bolts::rands::{Rand, StdRand};

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

#[derive(Debug, Clone)]
pub struct Game {
    pub field: Field,
    remaining_pieces: Vec<Piece>,
    pub status: Status,
}

impl Game {
    /// Starts a new game
    pub fn new(starting_player: Player) -> Self {
        let remaining_pieces = (0..16).map(Piece::new_with_props).collect();

        Self {
            remaining_pieces,
            field: Field::new(),
            status: Status::InitialMove { starting_player },
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
        if self.running() {
            println!("{:?}, move!", self.player());
            if let Some(piece) = self.next_piece() {
                print!("Your piece is: ");
                piece.pp();
                println!();
            }
        } else if let Some(winner) = self.winner() {
            println!("{:?} won!", winner);
        } else {
            println!("Game ended in a draw!");
        }

        println!("\nField:");
        self.field.pp();
        println!();

        if !self.remaining_pieces().is_empty() {
            println!("\nRemaining Pieces:");
            self.pp_remaining_pieces();
        }
    }

    pub fn pp_remaining_pieces(&self) {
        for (i, piece) in self.remaining_pieces().iter().enumerate() {
            if i > 0 && (i) % 3 == 0 {
                println!();
            }
            print!("{}: ", i);
            if i < 10 {
                // padding for low numbers
                print!(" ");
            }
            piece.pp();
            if i < (Field::SIZE * Field::SIZE) - 1 && (i + 1) % 3 != 0 {
                print!(", ");
            }
        }
        println!();
    }

    /// Starts a new game with a random player
    pub fn with_rand_player(rng: &mut StdRand) -> Self {
        if rng.next() % 2 == 0 {
            Self::new(Player::PlayerOne)
        } else {
            Self::new(Player::PlayerTwo)
        }
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
        if let Status::Move {
            next_player: player,
            next_piece: piece,
        } = self.status
        {
            self.field.put(pos, piece)?;
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

    pub fn last_move(&mut self, pos: Pos) -> Result<(), ()> {
        if let Status::Move {
            next_player: player,
            next_piece: piece,
        } = self.status
        {
            self.field.put(pos, piece)?;
            if self.field.check_field_for_win() {
                self.status = Status::Won { winner: player }
            } else {
                self.status = Status::Draw {
                    last_player: player,
                }
            };
            Ok(())
        } else {
            Err(())
        }
    }

    /// Undo the latest move
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

    pub fn unmove_initial(&mut self) {
        assert!(
            self.is_initial_move(),
            "used unmove_initial to unmove non-initial move"
        );
        self.remaining_pieces = (0..16).map(Piece::new_with_props).collect();
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
