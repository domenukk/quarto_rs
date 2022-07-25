/// Taken from <https://github.com/AFLplusplus/LibAFL/blob/main/libafl/src/bolts/rands.rs>
#[derive(Copy, Clone, Debug)]
pub struct RomuDuoJrRand {
    x_state: u64,
    y_state: u64,
}

impl RomuDuoJrRand {
    /// Creates a new `RomuDuoJrRand` with the given seed.
    #[must_use]
    pub fn with_seed(seed: u64) -> Self {
        let mut rand = Self {
            x_state: 0,
            y_state: 0,
        };
        rand.set_seed(seed);
        rand
    }

    fn set_seed(&mut self, seed: u64) {
        self.x_state = seed ^ 0x12345;
        self.y_state = seed ^ 0x6789A;
    }

    #[inline]
    #[allow(clippy::unreadable_literal)]
    pub fn next(&mut self) -> u64 {
        let xp = self.x_state;
        self.x_state = 15241094284759029579_u64.wrapping_mul(self.y_state);
        self.y_state = self.y_state.wrapping_sub(xp).rotate_left(27);
        xp
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rng() {
        use crate::rng::RomuDuoJrRand;
        let mut rng = RomuDuoJrRand::with_seed(13371339);

        let _ = rng.next();
    }
}
