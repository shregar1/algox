use crate::abstraction::AlgorithmTrait;

/// Trait for sorting algorithms.
pub trait SortingAlgorithmTrait<T>: AlgorithmTrait {
    /// Sort slice in-place.
    fn sort(&self, slice: &mut [T]);
}
