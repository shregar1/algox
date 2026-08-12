use crate::abstraction::AlgorithmTrait;
use super::abstraction::IndexingAlgorithmTrait;
use std::collections::BTreeMap;

pub struct BTreeIndex<K, V> {
    tree: BTreeMap<K, V>,
}

impl<K: Ord, V> BTreeIndex<K, V> {
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.tree.insert(key, value)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.tree.get(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.tree.remove(key)
    }

    pub fn len(&self) -> usize {
        self.tree.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    pub fn clear(&mut self) {
        self.tree.clear();
    }
}

impl<K: Ord, V> Default for BTreeIndex<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> AlgorithmTrait for BTreeIndex<K, V> {
    fn name(&self) -> &'static str {
        "btree"
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

impl<K: Ord, V> IndexingAlgorithmTrait<K, V> for BTreeIndex<K, V> {
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
    fn test_btree_index() {
        let mut index = BTreeIndex::new();
        index.insert("user1", 100);
        index.insert("user2", 200);
        assert_eq!(index.get(&"user1"), Some(&100));
        assert_eq!(index.remove(&"user1"), Some(100));
        assert_eq!(index.get(&"user1"), None);
    }
}
