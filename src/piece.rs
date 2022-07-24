use std::fmt::Formatter;

/// A quarto piece.
#[derive(Default, PartialEq, Eq, Copy, Clone)]
pub struct Piece {
    pub properties: u8,
}

impl std::fmt::Debug for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.pp_write(f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Property {
    Tall = 1 << 0,
    Round = 1 << 1,
    Full = 1 << 2,
    Light = 1 << 3,
}

impl Piece {
    #[must_use]
    pub const fn new() -> Self {
        Self::new_with_props(0)
    }

    #[must_use]
    pub const fn new_with_props(props: u8) -> Self {
        assert!(props >> 4 == 0, "top bits should be clear");
        let props = props & !(props << 4);
        Piece { properties: props }
    }

    pub fn set(&mut self, prop: Property, val: bool) {
        if val {
            self.properties |= prop as u8;
            self.properties &= !((prop as u8) << 4);
        } else {
            self.properties &= !(prop as u8);
            self.properties |= (prop as u8) << 4;
        }
    }

    pub fn get(self, prop: Property) -> bool {
        (self.properties & prop as u8) != 0
    }

    pub fn pp_write(self, f: &mut Formatter) -> std::fmt::Result {
        f.write_str("[")?;
        if self.get(Property::Tall) {
            f.write_str("✋")?;
            //f.write_str("⬆️")?;
        } else {
            f.write_str("🤏")?;
            //f.write_str("⬇️")?;
        }
        if self.get(Property::Round) {
            f.write_str("🟠")?;
        } else {
            write!(f, "🔶")?;
        }
        if self.get(Property::Full) {
            f.write_str("🔴")?;
        } else {
            f.write_str("⭕")?;
        }
        if self.get(Property::Light) {
            //f.write_str("🏳️")?;
            f.write_str("⬜")?;
        } else {
            f.write_str("🏴")?;
            //f.write_str("⬛")?;
        }
        f.write_str("]")
    }

    /// Pretty-print a piece
    pub fn pp(self) {
        print!("[");
        if self.get(Property::Tall) {
            print!("✋");
            //print!("️⬆️");
        } else {
            print!("🤏");
            //print!("⬇️");
        }
        if self.get(Property::Light) {
            //print!("🏳️");
            print!("⬜");
        } else {
            //print!("🏴");
            print!("⬛");
        }
        if self.get(Property::Round) {
            print!("🟠");
        } else {
            print!("🔶");
        }
        if self.get(Property::Full) {
            print!("🔴");
        } else {
            print!("⭕");
        }
        print!("]");
    }
}
