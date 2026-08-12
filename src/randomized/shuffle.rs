use crate::abstraction::AlgorithmTrait;
use super::abstraction::RandomizedAlgorithmTrait;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::rngs::StdRng;

/// Fisher-Yates shuffle — uniform random permutation in O(n).
pub struct Shuffle;

impl Shuffle {
    /// Shuffles `slice` in-place using system entropy (`rand::thread_rng()`).
    pub fn shuffle<T>(slice: &mut [T]) {
        let mut rng = rand::thread_rng();
        slice.shuffle(&mut rng);
    }

    /// Shuffles `slice` in-place using a deterministic 64-bit seed (ideal for reproducible tests or simulations).
    pub fn shuffle_with_seed<T>(slice: &mut [T], seed: u64) {
        let mut rng = StdRng::seed_from_u64(seed);
        slice.shuffle(&mut rng);
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
    fn test_shuffle_system_rng() {
        let mut arr: Vec<i32> = (0..10).collect();
        let original = arr.clone();
        Shuffle::shuffle(&mut arr);
        let mut sorted = arr.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, original);
    }

    #[test]
    fn test_shuffle_changes_order() {
        let mut arr: Vec<i32> = (0..20).collect();
        let original = arr.clone();
        Shuffle::shuffle_with_seed(&mut arr, 12345);
        assert_ne!(arr, original);
    }
}
