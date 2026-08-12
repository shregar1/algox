use crate::abstraction::AlgorithmTrait;

/// Trait for caching algorithms.
pub trait CacheAlgorithmTrait<K, V>: AlgorithmTrait {
    /// Get reference to value by key, updating access statistics if applicable.
    fn get(&mut self, key: &K) -> Option<&V>;

    /// Peek value by key without updating eviction ordering/frequency.
    fn peek(&self, key: &K) -> Option<&V>;

    /// Insert a key-value pair into cache. Returns old value if present.
    fn put(&mut self, key: K, value: V) -> Option<V>;

    /// Remove entry by key. Returns value if present.
    fn remove(&mut self, key: &K) -> Option<V>;

    /// Maximum capacity of the cache.
    fn capacity(&self) -> usize;
}
