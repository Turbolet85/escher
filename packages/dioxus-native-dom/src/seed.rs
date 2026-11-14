//! The run's seeded source of randomness: one generator a document's components draw from in
//! the order their draws happen, seeded once for the run.
//!
//! A harness-built document reads the [`SeededRandom`] its vdom's root provides — the seed its
//! builder gave, else [`DEFAULT_SEED`] — so two runs given one seed read the same state. An app
//! reads it through [`use_seeded_random`]. The generator holds a number and nothing else: no
//! content, and it logs nothing.

use std::cell::Cell;
use std::hash::{BuildHasher, RandomState};
use std::rc::Rc;

use dioxus_core::{try_consume_context, use_hook};

/// The seed a harness-built document reads when the builder of its vdom gives none.
pub const DEFAULT_SEED: u64 = 0;

const GOLDEN_GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;
const MIX_1: u64 = 0xBF58_476D_1CE4_E5B9;
const MIX_2: u64 = 0x94D0_49BB_1331_11EB;

/// A splitmix64 generator. Every clone shares one state, so the components of a document that
/// hold it draw from one stream.
#[derive(Clone)]
pub struct SeededRandom {
    state: Rc<Cell<u64>>,
}

impl std::fmt::Debug for SeededRandom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SeededRandom")
    }
}

impl SeededRandom {
    /// A generator whose stream is the one `seed` gives.
    pub fn from_seed(seed: u64) -> Self {
        Self {
            state: Rc::new(Cell::new(seed)),
        }
    }

    /// A generator seeded from the host: what an app reads outside a run, where no root
    /// provides one.
    fn from_host() -> Self {
        Self::from_seed(RandomState::new().hash_one(GOLDEN_GAMMA))
    }

    /// The stream's next value.
    pub fn next_u64(&self) -> u64 {
        let state = self.state.get().wrapping_add(GOLDEN_GAMMA);
        self.state.set(state);
        let mut z = state;
        z = (z ^ (z >> 30)).wrapping_mul(MIX_1);
        z = (z ^ (z >> 27)).wrapping_mul(MIX_2);
        z ^ (z >> 31)
    }

    /// The stream's next value in `0..bound`, by the high half of its product with `bound`;
    /// `0` for a `bound` of `0`.
    pub fn below(&self, bound: u64) -> u64 {
        let product = u128::from(self.next_u64()) * u128::from(bound);
        u64::try_from(product >> 64).unwrap_or(0)
    }
}

/// The run's generator, as the component calling it reads it: the one the root provides, else
/// one seeded from the host. A hook: call it on every render, in a fixed place.
pub fn use_seeded_random() -> SeededRandom {
    use_hook(|| try_consume_context::<SeededRandom>().unwrap_or_else(SeededRandom::from_host))
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
        let random = SeededRandom::from_seed(0);
        for (row, expected) in SEED_ZERO.iter().enumerate() {
            assert!(random.next_u64() == *expected, "row {row}");
        }
    }

    #[test]
    fn one_seed_reads_one_stream() {
        let a = SeededRandom::from_seed(7);
        let b = SeededRandom::from_seed(7);
        for draw in 0..1_000 {
            assert!(a.next_u64() == b.next_u64(), "draw {draw}");
        }
    }

    #[test]
    fn two_seeds_differ_at_the_first_draw() {
        assert_ne!(
            SeededRandom::from_seed(7).next_u64(),
            SeededRandom::from_seed(8).next_u64()
        );
    }

    #[test]
    fn a_clone_draws_from_the_shared_stream() {
        let random = SeededRandom::from_seed(0);
        let clone = random.clone();
        assert!(clone.next_u64() == SEED_ZERO[0]);
        assert!(random.next_u64() == SEED_ZERO[1]);
        assert!(clone.next_u64() == SEED_ZERO[2]);
    }

    #[test]
    fn below_stays_inside_its_bound() {
        const BOUNDS: [u64; 3] = [1, 6, u64::MAX];
        assert_eq!(BOUNDS.len(), 3);
        let random = SeededRandom::from_seed(7);
        for (row, bound) in BOUNDS.iter().enumerate() {
            for _ in 0..100 {
                assert!(random.below(*bound) < *bound, "row {row}");
            }
        }
        assert_eq!(random.below(0), 0);
    }

    #[test]
    fn debug_writes_no_state() {
        assert_eq!(format!("{:?}", SeededRandom::from_seed(7)), "SeededRandom");
    }
}
