use std::cell::Cell;

/// XorShift64
pub struct X64 {
    state: Cell<u64>,
}

impl X64 {
    pub fn new(seed: u64) -> Self {
        Self {
            state: Cell::new(seed),
        }
    }

    pub fn get_rand(&self) -> u64 {
        let mut x = self.state.get();

        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;

        self.state.set(x);

        return x;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rng() {
        use crate::rng::X64;
        let rng = X64::new(13371339);

        let _ = rng.get_rand();
    }
}
