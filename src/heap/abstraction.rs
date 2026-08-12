use crate::abstraction::AlgorithmTrait;

/// Trait for heap data structures.
pub trait HeapAlgorithmTrait<T>: AlgorithmTrait {
    /// Push an item onto the heap.
    fn push(&mut self, item: T);

    /// Pop the top item from the heap.
    fn pop(&mut self) -> Option<T>;

    /// Peek the top item without removing it.
    fn peek(&self) -> Option<&T>;
}
