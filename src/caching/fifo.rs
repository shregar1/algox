use crate::abstraction::AlgorithmTrait;
use super::abstraction::CacheAlgorithmTrait;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::num::NonZeroUsize;

pub struct FifoCache<K, V> {
    capacity: NonZeroUsize,
    map: HashMap<K, V>,
    queue: VecDeque<K>,
}

impl<K: Eq + Hash + Clone, V> FifoCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        let capacity = NonZeroUsize::new(capacity.max(1)).expect("capacity must be non-zero");
        Self {
            capacity,
            map: HashMap::with_capacity(capacity.get()),
            queue: VecDeque::with_capacity(capacity.get()),
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity.get()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn peek(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        if let Some(existing) = self.map.get_mut(&key) {
            return Some(std::mem::replace(existing, value));
        }

        if self.map.len() >= self.capacity.get() {
            if let Some(oldest) = self.queue.pop_front() {
                self.map.remove(&oldest);
            }
        }

        self.queue.push_back(key.clone());
        self.map.insert(key, value);
        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(val) = self.map.remove(key) {
            self.queue.retain(|k| k != key);
            Some(val)
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.queue.clear();
    }
}

impl<K: Eq + Hash + Clone, V> AlgorithmTrait for FifoCache<K, V> {
    fn name(&self) -> &'static str {
        "fifo_cache"
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

impl<K: Eq + Hash + Clone, V> CacheAlgorithmTrait<K, V> for FifoCache<K, V> {
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
    fn test_fifo_eviction() {
        let mut cache: FifoCache<&str, i32> = FifoCache::new(2);
        cache.put("a", 1);
        cache.put("b", 2);
        cache.get(&"a"); // Reading "a" should not save it from FIFO eviction
        cache.put("c", 3);

        assert!(!cache.contains(&"a")); // "a" was inserted first, so evicted
        assert!(cache.contains(&"b"));
        assert!(cache.contains(&"c"));
    }
}
