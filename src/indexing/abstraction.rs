use crate::abstraction::AlgorithmTrait;

/// Trait for indexing data structures.
pub trait IndexingAlgorithmTrait<K, V>: AlgorithmTrait {
    /// Insert a key-value mapping into index.
    fn insert(&mut self, key: K, value: V) -> Option<V>;

    /// Lookup value by key.
    fn get(&self, key: &K) -> Option<&V>;

    /// Remove entry by key.
    fn remove(&mut self, key: &K) -> Option<V>;
}
