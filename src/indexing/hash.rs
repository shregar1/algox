use crate::abstraction::AlgorithmTrait;
use super::abstraction::IndexingAlgorithmTrait;
use std::collections::HashMap;
use std::hash::Hash;

pub struct HashIndex<K, V> {
    map: HashMap<K, V>,
}

impl<K: Eq + Hash, V> HashIndex<K, V> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.map.insert(key, value)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}

impl<K: Eq + Hash, V> Default for HashIndex<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Eq + Hash, V> AlgorithmTrait for HashIndex<K, V> {
    fn name(&self) -> &'static str {
        "hash"
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn clear(&mut self) {
        self.clear();
    }
}

impl<K: Eq + Hash, V> IndexingAlgorithmTrait<K, V> for HashIndex<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_index() {
        let mut index = HashIndex::new();
        index.insert("user1", 100);
        index.insert("user2", 200);
        assert_eq!(index.get(&"user1"), Some(&100));
        assert_eq!(index.remove(&"user1"), Some(100));
        assert_eq!(index.get(&"user1"), None);
    }
}
