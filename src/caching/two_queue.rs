use crate::abstraction::AlgorithmTrait;
use super::abstraction::CacheAlgorithmTrait;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::num::NonZeroUsize;

/// Two-Queue (2Q) Cache algorithm.
/// Divides capacity between a FIFO queue for single accesses and an LRU queue for frequent accesses.
pub struct TwoQueueCache<K, V> {
    capacity: NonZeroUsize,
    in_capacity: usize,
    fifo_in: VecDeque<K>,
    lru_map: HashMap<K, V>,
    lru_order: Vec<K>,
}

impl<K: Eq + Hash + Clone, V> TwoQueueCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        let cap = capacity.max(2);
        let non_zero_cap = NonZeroUsize::new(cap).expect("non-zero capacity");
        let in_capacity = (cap / 2).max(1);

        Self {
            capacity: non_zero_cap,
            in_capacity,
            fifo_in: VecDeque::with_capacity(in_capacity),
            lru_map: HashMap::with_capacity(cap),
            lru_order: Vec::with_capacity(cap),
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity.get()
    }

    pub fn len(&self) -> usize {
        self.lru_map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.lru_map.is_empty()
    }

    pub fn contains(&self, key: &K) -> bool {
        self.lru_map.contains_key(key)
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.lru_map.contains_key(key) {
            self.touch(key);
            self.lru_map.get(key)
        } else {
            None
        }
    }

    pub fn peek(&self, key: &K) -> Option<&V> {
        self.lru_map.get(key)
    }

    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        let old = self.lru_map.insert(key.clone(), value);
        if old.is_some() {
            self.touch(&key);
        } else {
            if self.lru_map.len() > self.capacity.get() {
                if let Some(victim) = self.fifo_in.pop_front() {
                    self.lru_map.remove(&victim);
                    self.lru_order.retain(|k| k != &victim);
                } else if let Some(victim) = self.lru_order.first().cloned() {
                    self.lru_order.remove(0);
                    self.lru_map.remove(&victim);
                }
            }
            if self.fifo_in.len() >= self.in_capacity {
                self.fifo_in.pop_front();
            }
            self.fifo_in.push_back(key.clone());
            self.lru_order.push(key);
        }
        old
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let val = self.lru_map.remove(key);
        if val.is_some() {
            self.fifo_in.retain(|k| k != key);
            self.lru_order.retain(|k| k != key);
        }
        val
    }

    pub fn clear(&mut self) {
        self.lru_map.clear();
        self.fifo_in.clear();
        self.lru_order.clear();
    }

    fn touch(&mut self, key: &K) {
        if let Some(pos) = self.lru_order.iter().position(|k| k == key) {
            let k = self.lru_order.remove(pos);
            self.lru_order.push(k);
        }
    }
}

impl<K: Eq + Hash + Clone, V> AlgorithmTrait for TwoQueueCache<K, V> {
    fn name(&self) -> &'static str {
        "two_queue_cache"
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

impl<K: Eq + Hash + Clone, V> CacheAlgorithmTrait<K, V> for TwoQueueCache<K, V> {
    fn get(&mut self, key: &K) -> Option<&V> {
        self.get(key)
    }

    fn peek(&self, key: &K) -> Option<&V> {
        self.peek(key)
    }

    fn put(&mut self, key: K, value: V) -> Option<V> {
        self.put(key, value)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }

    fn capacity(&self) -> usize {
        self.capacity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_queue() {
        let mut cache: TwoQueueCache<&str, i32> = TwoQueueCache::new(3);
        cache.put("a", 1);
        cache.put("b", 2);
        cache.put("c", 3);
        assert_eq!(cache.get(&"a"), Some(&1));
        assert_eq!(cache.len(), 3);
    }
}
