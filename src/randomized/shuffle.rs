use crate::abstraction::AlgorithmTrait;
use super::abstraction::RandomizedAlgorithmTrait;

/// Fisher-Yates shuffle — uniform random permutation in O(n).
pub struct Shuffle;

impl Shuffle {
    /// Shuffles `slice` in-place using a simple LCG for determinism in tests.
    /// For production use, replace `lcg_next` with a cryptographic or OS RNG.
    pub fn shuffle_with_seed<T>(slice: &mut [T], seed: u64) {
        let n = slice.len();
        if n <= 1 { return; }
        let mut rng = seed;
        for i in (1..n).rev() {
            rng = Self::lcg_next(rng);
            let j = (rng as usize) % (i + 1);
            slice.swap(i, j);
        }
    }

    fn lcg_next(state: u64) -> u64 {
        state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407)
    }
}

impl AlgorithmTrait for Shuffle {
    fn name(&self) -> &'static str {
        "fisher_yates_shuffle"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl RandomizedAlgorithmTrait for Shuffle {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shuffle_is_permutation() {
        let mut arr: Vec<i32> = (0..10).collect();
        let original = arr.clone();
        Shuffle::shuffle_with_seed(&mut arr, 42);
        let mut sorted = arr.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, original);
    }

    #[test]
    fn test_shuffle_changes_order() {
        let mut arr: Vec<i32> = (0..20).collect();
        let original = arr.clone();
        Shuffle::shuffle_with_seed(&mut arr, 12345);
        // Very unlikely to be identical after shuffling 20 elements
        assert_ne!(arr, original);
    }
}
