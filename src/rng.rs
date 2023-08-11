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

    /// Gets a value below the given 64 bit val (inclusive)
    pub fn below(&mut self, upper_bound_excl: u64) -> u64 {
        if upper_bound_excl <= 1 {
            return 0;
        }

        /*
        Modulo is biased - we don't want our fuzzing to be biased so let's do it
        right. See
        https://stackoverflow.com/questions/10984974/why-do-people-say-there-is-modulo-bias-when-using-a-random-number-generator
        */
        let mut unbiased_rnd: u64;
        loop {
            unbiased_rnd = self.next();
            if unbiased_rnd < (u64::MAX - (u64::MAX % upper_bound_excl)) {
                break;
            }
        }

        unbiased_rnd % upper_bound_excl
    }

    /// Choose an item at random from the given iterator, sampling uniformly.
    ///
    /// Note: the runtime cost is bound by the iterator's [`nth`][`Iterator::nth`] implementation
    ///  * For `Vec`, slice, array, this is O(1)
    ///  * For `HashMap`, `HashSet`, this is O(n)
    pub fn choose<I, E, T>(&mut self, from: I) -> T
    where
        I: IntoIterator<Item = T, IntoIter = E>,
        E: ExactSizeIterator + Iterator<Item = T>,
    {
        // create iterator
        let mut iter = from.into_iter();

        // make sure there is something to choose from
        debug_assert!(iter.len() > 0, "choosing from an empty iterator");

        // pick a random, valid index
        #[allow(clippy::cast_possible_truncation)]
        let index = self.below(iter.len() as u64) as usize;

        // return the item chosen
        iter.nth(index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rng() {
        use crate::rng::RomuDuoJrRand;
        let mut rng = RomuDuoJrRand::with_seed(13371339);

        _ = rng.next();
    }
}
