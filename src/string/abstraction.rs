use crate::abstraction::AlgorithmTrait;

/// Trait for string matching / similarity algorithms.
pub trait StringAlgorithmTrait: AlgorithmTrait {
    /// Compute similarity metric or match.
    fn compute(&self, a: &str, b: &str) -> usize;
}
