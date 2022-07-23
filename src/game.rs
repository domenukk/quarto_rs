use crate::{field::{Field, Pos}, piece::Piece};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Player {
    PlayerOne,
    PlayerTwo,
}

impl Player {
    pub fn next(&self) -> Self {
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
    Draw {last_move: Player},
}

#[derive(Debug, Clone)]
pub struct Game {
    pub field: Field,
    remaining_pieces: Vec<Piece>,
    pub status: Status,
}

impl Game {

    pub fn remaining_pieces(&self) -> &[Piece] {
        &self.remaining_pieces
    }

    pub fn new(starting_player: Player) -> Self {
        let remaining_pieces = (0..16).map(Piece::new_with_props).collect();

        Self {
            remaining_pieces,
            field: Field::new(),
            status: Status::InitialMove { starting_player },
        }
    }

    /// Gives the initial piece to the opponent, as we do not actually put a piece onto the field
    /// in the first turn.
    pub fn initial_move(&mut self, next_piece: Piece) -> Result<(), ()> {
        if let Status::InitialMove {starting_player} = self.status {
            self.status = Status::Move { next_player: starting_player.next(), next_piece };
            Ok(())
        } else {
            Err(())
        }
    }

    /// Next move, actually put a piece on the field, and give the next piece to the opponent or
    /// checks if a player won..
    pub fn do_move(&mut self, pos: Pos, next_piece: Piece) -> Result<(), ()> {
        if let Status::Move { next_player: player, next_piece: piece } = self.status {

            self.field.put(pos, piece)?;
            if self.field.check_field_for_win() {
                self.status = Status::Won{winner: player}
            } else {
                self.status = Status::Move{next_player: player.next(), next_piece}
            };
            Ok(())

        } else {
            Err(())
        }
    }

    pub fn last_move(&mut self, pos: Pos) -> Result<(), ()> {
         if let Status::Move { next_player: player, next_piece: piece } = self.status {
            self.field.put(pos, piece)?;
            if self.field.check_field_for_win() {
                self.status = Status::Won{winner: player}
            } else {
                self.status = Status::Draw{last_player: player}
            };
         } else {
             Err(())
         }
    }


    /// Undo the latest move
    pub fn unmove(&mut status, last_pos: Pos) {
        let next_player = match self.status() {
            Status::IntialMove { .. } => panic!("Can't unmove an initial move"),
            Status::Won{winner} => winner,
            Status::Draw{last_player} => last_player,
            Status::Move{next_player, ..} => next_player,
        }

        // There are only two players, so next is also prev.
        let prev_player = next_player.next()


        let last_piece = self.field[last_pos.0][last_pos.1].unwrap();
        self.field[last_pos.0][last_pos.1] = None;
        self.remaining_pieces.push(last_piece);

        if self.remaining_pieces.len() == Field::SIZE * Field::SIZE {
            self.status = Status::InitialMove {
                starting_player: next_player
            }
        } else {
            self.status = Status::Move {
                next_piece: last_piece,
                next_player: prev_player,
            }
        };
        Ok(())
    }

}
