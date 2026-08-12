use crate::abstraction::AlgorithmTrait;
use super::abstraction::CacheAlgorithmTrait;
use std::collections::HashMap;
use std::hash::Hash;
use std::num::NonZeroUsize;

struct Entry<V> {
    value: V,
    freq: usize,
}

pub struct LfuCache<K, V> {
    capacity: NonZeroUsize,
    map: HashMap<K, Entry<V>>,
}

impl<K: Eq + Hash + Clone, V> LfuCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        let capacity = NonZeroUsize::new(capacity.max(1)).expect("capacity must be non-zero");
        Self {
            capacity,
            map: HashMap::with_capacity(capacity.get()),
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
        if let Some(entry) = self.map.get_mut(key) {
            entry.freq += 1;
            Some(&entry.value)
        } else {
            None
        }
    }

    pub fn peek(&self, key: &K) -> Option<&V> {
        self.map.get(key).map(|e| &e.value)
    }

    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        if let Some(entry) = self.map.get_mut(&key) {
            entry.freq += 1;
            let old = std::mem::replace(&mut entry.value, value);
            return Some(old);
        }

        if self.map.len() >= self.capacity.get() {
            // Find key with minimum frequency
            if let Some(lfu_key) = self
                .map
                .iter()
                .min_by_key(|(_, entry)| entry.freq)
                .map(|(k, _)| k.clone())
            {
                self.map.remove(&lfu_key);
            }
        }

        self.map.insert(key, Entry { value, freq: 1 });
        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key).map(|e| e.value)
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}

impl<K: Eq + Hash + Clone, V> AlgorithmTrait for LfuCache<K, V> {
    fn name(&self) -> &'static str {
        "lfu_cache"
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

impl<K: Eq + Hash + Clone, V> CacheAlgorithmTrait<K, V> for LfuCache<K, V> {
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
    fn test_lfu_eviction() {
        let mut cache: LfuCache<&str, i32> = LfuCache::new(2);
        cache.put("a", 1);
        cache.put("b", 2);

        // Access "a" twice, "b" once
        cache.get(&"a");
        cache.get(&"a");

        // Insert "c", should evict least frequently used "b"
        cache.put("c", 3);
        assert!(cache.contains(&"a"));
        assert!(!cache.contains(&"b"));
        assert!(cache.contains(&"c"));
    }
}
