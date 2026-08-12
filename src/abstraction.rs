//! Common algorithm abstraction trait for `rivex-algorithm`.

/// Common trait for algorithm data structures and utilities.
pub trait AlgorithmTrait {
    /// Returns the name identifier of the algorithm.
    fn name(&self) -> &'static str;

    /// Returns the current number of elements/items stored or tracked by the algorithm.
    fn len(&self) -> usize;

    /// Returns `true` if the algorithm data structure is currently empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears or resets the state of the algorithm data structure.
    fn clear(&mut self);
}
