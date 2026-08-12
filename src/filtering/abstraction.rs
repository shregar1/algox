use crate::abstraction::AlgorithmTrait;

/// Trait for filter algorithms.
pub trait FilterAlgorithmTrait: AlgorithmTrait {
    /// Add item to filter.
    fn add(&mut self, item: &[u8]);

    /// Check if item is present in filter.
    fn contains(&self, item: &[u8]) -> bool;
}
