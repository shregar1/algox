use crate::abstraction::AlgorithmTrait;

/// Trait for ring and queue buffer data structures.
pub trait BufferAlgorithmTrait<T>: AlgorithmTrait {
    /// Push item to back.
    fn push_back(&mut self, value: T) -> Result<(), T>;

    /// Pop item from front.
    fn pop_front(&mut self) -> Option<T>;

    /// Peek front item.
    fn front(&self) -> Option<&T>;

    /// Peek back item.
    fn back(&self) -> Option<&T>;

    /// Capacity of buffer.
    fn capacity(&self) -> usize;
}
