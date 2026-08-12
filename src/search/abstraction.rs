use crate::abstraction::AlgorithmTrait;

/// Trait for search algorithms.
pub trait SearchAlgorithmTrait<T>: AlgorithmTrait {
    /// Search for target element in slice. Returns index if found.
    fn search(&self, slice: &[T], target: &T) -> Option<usize>;
}
