use std::iter::Enumerate;

use crate::piece::Piece;

#[derive(Debug, Default, Clone)]
pub struct Field {
    // The field of a quarto game.
    field: [[Option<Piece>; Self::SIZE]; Self::SIZE],
}

pub type Pos = (usize, usize);

impl Field {
    pub const SIZE: usize = 4;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn put(&mut self, pos: Pos, piece: Piece) -> Result<(), ()> {
        if self.field[pos.0][pos.1].is_none() {
            self.field[pos.0][pos.1] = Some(piece);
            return Ok(());
        }
        Err(())
    }

    pub fn get(&self, pos: Pos) -> Option<Piece> {
        self.field[pos.0][pos.1]
    }

    /// Clear at a position, returning the current piece at this point
    pub fn clear(&mut self, pos: Pos) -> Option<Piece> {
        let ret = self.get(pos);
        self.field[pos.0][pos.1] = None;
        ret
    }

    /// Checks if the win condition on this field is fulfilled.
    pub fn check_field_for_win(&self) -> bool {
        for row in &self.field {
            if Self::check_array_for_win(row) {
                return true;
            }
        }

        for column_idx in 0..Self::SIZE {
            let col: Vec<Option<Piece>> = self.field.iter().map(|x| x[column_idx]).collect();
            if Self::check_array_for_win(&col) {
                return true;
            }
        }

        let diagonal: Vec<Option<Piece>> = (0..Self::SIZE).map(|x| self.field[x][x]).collect();
        if Self::check_array_for_win(&diagonal) {
            return true;
        }

        let diagonal: Vec<Option<Piece>> = (0..Self::SIZE)
            .map(|x| self.field[x][(Self::SIZE - 1) - x])
            .collect();
        if Self::check_array_for_win(&diagonal) {
            return true;
        }

        for i in 0..(Self::SIZE - 1) {
            let mut flattened_square = [None; 4];
            for k in 0..(Self::SIZE - 1) {
                //for l in 0..2 {
                //flattened_square[l] = self.field[i][k + l]
                //}
                flattened_square[..2].copy_from_slice(&self.field[i][k..(2 + k)]);
                //for l in 0..2 {
                //flattened_square[l + 2] = self.field[i + 1][k + l]
                //}
                flattened_square[2..(2 + 2)].copy_from_slice(&self.field[i + 1][k..(2 + k)]);
                if Self::check_array_for_win(&flattened_square) {
                    return true;
                }
            }
        }

        false
    }

    // Associated helper function to determine if a given line of pieces fulfills a win condition
    fn check_array_for_win(ary: &[Option<Piece>]) -> bool {
        assert!(ary.len() == 4);

        let mut ret = std::u8::MAX;

        for piece in ary {
            if let Some(piece) = piece {
                ret &= piece.properties;
            } else {
                return false;
            }
        }

        ret != 0
    }

    pub fn empty_spaces(&self) -> Vec<Pos> {
        let mut ret = Vec::with_capacity(16);

        for x in 0..Self::SIZE {
            for y in 0..Self::SIZE {
                if self.field[x][y].is_none() {
                    ret.push((x, y));
                }
            }
        }

        ret
    }

    pub fn pp(&self) {
        for (i, row) in (&self.field).iter().enumerate() {
            for (_k, val) in (row).iter().enumerate() {
                if i == 0 {
                    println!("\n ------   -------   ------   ------- ");
                } else if i < Self::SIZE - 1 {
                    print!(" | ");
                }
                if let Some(val) = val {
                    val.pp();
                } else {
                    print!("      ");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        field::Field,
        piece::{Piece, Property},
    };
    const TEST_LIGHT_TALL: Piece =
        Piece::new_with_props(Property::Tall as u8 | Property::Light as u8);
    const TEST_DARK_SHORT: Piece = Piece::new();

    #[test]
    fn test_squares() {
        let mut field = Field::new();

        field.put((0, 0), TEST_LIGHT_TALL).unwrap();
        field.put((0, 1), TEST_LIGHT_TALL).unwrap();
        field.put((1, 0), TEST_LIGHT_TALL).unwrap();

        assert!(!field.check_field_for_win());

        field.put((1, 1), TEST_LIGHT_TALL).unwrap();

        assert!(field.check_field_for_win());
    }

    #[test]
    fn test_wrong_prop_square() {
        let mut field = Field::new();

        field.put((0, 0), TEST_LIGHT_TALL).unwrap();
        field.put((0, 1), TEST_DARK_SHORT).unwrap();
        field.put((1, 0), TEST_LIGHT_TALL).unwrap();
        field.put((1, 1), TEST_LIGHT_TALL).unwrap();

        assert!(!field.check_field_for_win());
    }

    #[test]
    fn test_different_square() {
        let mut field = Field::new();

        field.put((2, 2), TEST_LIGHT_TALL).unwrap();
        field.put((2, 3), TEST_LIGHT_TALL).unwrap();
        field.put((3, 2), TEST_LIGHT_TALL).unwrap();
        field.put((3, 3), TEST_LIGHT_TALL).unwrap();

        assert!(field.check_field_for_win());
    }

    #[test]
    fn test_row() {
        let mut field = Field::new();

        field.put((0, 0), TEST_LIGHT_TALL).unwrap();
        field.put((0, 1), TEST_LIGHT_TALL).unwrap();
        field.put((0, 2), TEST_LIGHT_TALL).unwrap();

        assert!(!field.check_field_for_win());

        field.put((0, 3), TEST_LIGHT_TALL).unwrap();

        assert!(field.check_field_for_win());
    }

    #[test]
    fn test_wrong_prop_row() {
        let mut field = Field::new();

        field.put((0, 0), TEST_LIGHT_TALL).unwrap();
        field.put((0, 1), TEST_LIGHT_TALL).unwrap();
        field.put((0, 2), TEST_DARK_SHORT).unwrap();

        assert!(!field.check_field_for_win());

        field.put((0, 3), TEST_LIGHT_TALL).unwrap();

        assert!(!field.check_field_for_win());
    }

    #[test]
    fn test_col() {
        let mut field = Field::new();

        field.put((0, 0), TEST_LIGHT_TALL).unwrap();
        field.put((1, 0), TEST_LIGHT_TALL).unwrap();
        field.put((2, 0), TEST_LIGHT_TALL).unwrap();

        assert!(!field.check_field_for_win());

        field.put((3, 0), TEST_LIGHT_TALL).unwrap();

        assert!(field.check_field_for_win());
    }

    #[test]
    fn test_wrong_prop_col() {
        let mut field = Field::new();

        field.put((0, 0), TEST_LIGHT_TALL).unwrap();
        field.put((1, 0), TEST_LIGHT_TALL).unwrap();
        field.put((2, 0), TEST_DARK_SHORT).unwrap();

        assert!(!field.check_field_for_win());

        field.put((3, 0), TEST_LIGHT_TALL).unwrap();

        assert!(!field.check_field_for_win());
    }

    #[test]
    fn test_diag() {
        let mut field = Field::new();

        field.put((0, 0), TEST_LIGHT_TALL).unwrap();
        field.put((1, 1), TEST_LIGHT_TALL).unwrap();
        field.put((2, 2), TEST_LIGHT_TALL).unwrap();

        assert!(!field.check_field_for_win());

        field.put((3, 3), TEST_LIGHT_TALL).unwrap();

        assert!(field.check_field_for_win());
    }

    #[test]
    fn test_wrong_prop_diag() {
        let mut field = Field::new();

        field.put((0, 0), TEST_LIGHT_TALL).unwrap();
        field.put((1, 1), TEST_LIGHT_TALL).unwrap();
        field.put((2, 2), TEST_DARK_SHORT).unwrap();

        assert!(!field.check_field_for_win());

        field.put((3, 3), TEST_LIGHT_TALL).unwrap();

        assert!(!field.check_field_for_win());
    }

    #[test]
    fn test_other_diag() {
        let mut field = Field::new();

        field.put((3, 0), TEST_LIGHT_TALL).unwrap();
        field.put((2, 1), TEST_LIGHT_TALL).unwrap();
        field.put((1, 2), TEST_LIGHT_TALL).unwrap();

        assert!(!field.check_field_for_win());

        field.put((0, 3), TEST_LIGHT_TALL).unwrap();

        assert!(field.check_field_for_win());
    }
}
