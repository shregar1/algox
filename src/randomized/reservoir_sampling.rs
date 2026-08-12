use crate::abstraction::AlgorithmTrait;
use super::abstraction::RandomizedAlgorithmTrait;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

/// Reservoir Sampling — select k uniform random samples from a stream of unknown size.
pub struct ReservoirSampling;

impl ReservoirSampling {
    /// Returns `k` elements sampled uniformly at random from `stream` using system entropy.
    pub fn sample<T: Clone>(stream: &[T], k: usize) -> Vec<T> {
        if k == 0 || stream.is_empty() { return Vec::new(); }
        let k = k.min(stream.len());
        let mut reservoir: Vec<T> = stream[..k].to_vec();
        let mut rng = rand::thread_rng();

        for (i, item) in stream[k..].iter().enumerate() {
            let j = rng.gen_range(0..=(i + k));
            if j < k {
                reservoir[j] = item.clone();
            }
        }
        reservoir
    }

    /// Returns `k` elements sampled uniformly at random from `stream` using a deterministic seed.
    pub fn sample_with_seed<T: Clone>(stream: &[T], k: usize, seed: u64) -> Vec<T> {
        if k == 0 || stream.is_empty() { return Vec::new(); }
        let k = k.min(stream.len());
        let mut reservoir: Vec<T> = stream[..k].to_vec();
        let mut rng = StdRng::seed_from_u64(seed);

        for (i, item) in stream[k..].iter().enumerate() {
            let j = rng.gen_range(0..=(i + k));
            if j < k {
                reservoir[j] = item.clone();
            }
        }
        reservoir
    }
}

impl AlgorithmTrait for ReservoirSampling {
    fn name(&self) -> &'static str {
        "reservoir_sampling"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl RandomizedAlgorithmTrait for ReservoirSampling {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reservoir_size() {
        let stream: Vec<i32> = (0..100).collect();
        let sample = ReservoirSampling::sample_with_seed(&stream, 10, 99);
        assert_eq!(sample.len(), 10);
    }

    #[test]
    fn test_reservoir_system_rng() {
        let stream: Vec<i32> = (0..100).collect();
        let sample = ReservoirSampling::sample(&stream, 10);
        assert_eq!(sample.len(), 10);
    }

    #[test]
    fn test_reservoir_subset() {
        let stream: Vec<i32> = (0..50).collect();
        let sample = ReservoirSampling::sample_with_seed(&stream, 5, 7);
        for v in &sample {
            assert!(stream.contains(v));
        }
    }

    #[test]
    fn test_reservoir_k_larger_than_stream() {
        let stream = vec![1, 2, 3];
        let sample = ReservoirSampling::sample_with_seed(&stream, 10, 1);
        assert_eq!(sample.len(), 3);
    }
}
