/// A quarto piece.
#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub struct Piece {
    pub properties: u8,
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
}
