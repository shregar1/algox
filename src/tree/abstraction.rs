use crate::abstraction::AlgorithmTrait;

/// Trait for tree data structures.
pub trait TreeAlgorithmTrait<K, V>: AlgorithmTrait {
    /// Insert key-value pair into tree.
    fn insert(&mut self, key: K, value: V) -> Option<V>;

    /// Find value by key.
    fn get(&self, key: &K) -> Option<&V>;
}
