//! A splitmix64 generator for a seeded `Math.random`, written by hand with no generator crate:
//! <https://prng.di.unimi.it/splitmix64.c>.

const GOLDEN_GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;
const MIX_1: u64 = 0xBF58_476D_1CE4_E5B9;
const MIX_2: u64 = 0x94D0_49BB_1331_11EB;

/// Steps `state` and returns the stream's next value.
pub(crate) fn splitmix64_next(state: &mut u64) -> u64 {
    *state = state.wrapping_add(GOLDEN_GAMMA);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(MIX_1);
    z = (z ^ (z >> 27)).wrapping_mul(MIX_2);
    z ^ (z >> 31)
}

/// `x`'s top 53 bits over 2^53: a value in [0, 1).
pub(crate) fn unit_interval(x: u64) -> f64 {
    // Both sides are below 2^53, so each converts to `f64` exactly.
    (x >> 11) as f64 / (1u64 << 53) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    /// splitmix64's first outputs for seed 0, as its reference implementation prints them.
    const SEED_ZERO: [u64; 3] = [
        0xE220_A839_7B1D_CDAF,
        0x6E78_9E6A_A1B9_65F4,
        0x06C4_5D18_8009_454F,
    ];

    #[test]
    fn seed_zero_reads_the_reference_outputs() {
        assert_eq!(SEED_ZERO.len(), 3);
        let mut state = 0;
        for (row, expected) in SEED_ZERO.iter().enumerate() {
            assert!(splitmix64_next(&mut state) == *expected, "row {row}");
        }
    }

    #[test]
    fn the_unit_interval_holds_its_ends() {
        assert!(unit_interval(0) == 0.0);
        let top = unit_interval(u64::MAX);
        assert!(top < 1.0 && top > 0.999_999);
    }
}
