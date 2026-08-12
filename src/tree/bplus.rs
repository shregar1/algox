use crate::abstraction::AlgorithmTrait;
use super::abstraction::TreeAlgorithmTrait;
use std::collections::BTreeMap;

pub struct BPlusTree<K, V> {
    map: BTreeMap<K, V>,
}

impl<K: Ord, V> BPlusTree<K, V> {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.map.insert(key, value)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn range<R>(&self, range: R) -> impl Iterator<Item = (&K, &V)>
    where
        R: std::ops::RangeBounds<K>,
    {
        self.map.range(range)
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

impl<K: Ord, V> Default for BPlusTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> AlgorithmTrait for BPlusTree<K, V> {
    fn name(&self) -> &'static str {
        "bplus_tree"
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

impl<K: Ord, V> TreeAlgorithmTrait<K, V> for BPlusTree<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bplus_tree() {
        let mut tree = BPlusTree::new();
        tree.insert(1, "a");
        tree.insert(2, "b");
        tree.insert(3, "c");

        assert_eq!(tree.get(&2), Some(&"b"));
        let range: Vec<_> = tree.range(1..=2).map(|(k, v)| (*k, *v)).collect();
        assert_eq!(range, vec![(1, "a"), (2, "b")]);
    }
}
