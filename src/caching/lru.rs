use crate::abstraction::AlgorithmTrait;
use super::abstraction::CacheAlgorithmTrait;
use std::collections::HashMap;
use std::hash::Hash;
use std::num::NonZeroUsize;

pub struct LruCache<K, V> {
    capacity: NonZeroUsize,
    map: HashMap<K, V>,
    order: Vec<K>,
}

impl<K: Eq + Hash + Clone, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        let capacity = NonZeroUsize::new(capacity.max(1)).expect("capacity must be non-zero");
        Self {
            capacity,
            map: HashMap::with_capacity(capacity.get()),
            order: Vec::with_capacity(capacity.get()),
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

    pub fn is_full(&self) -> bool {
        self.map.len() >= self.capacity.get()
    }

    pub fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            self.touch(key);
            self.map.get(key)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.map.contains_key(key) {
            self.touch(key);
            self.map.get_mut(key)
        } else {
            None
        }
    }

    pub fn peek(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        let old = self.map.insert(key.clone(), value);
        if old.is_some() {
            self.touch(&key);
        } else {
            self.order.push(key);
            if self.map.len() > self.capacity.get() {
                if let Some(victim) = self.order.first().cloned() {
                    self.order.remove(0);
                    self.map.remove(&victim);
                }
            }
        }
        old
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let v = self.map.remove(key);
        if v.is_some() {
            self.order.retain(|k| k != key);
        }
        v
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.order.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.map.iter()
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.map.keys()
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.map.values()
    }

    fn touch(&mut self, key: &K) {
        if let Some(pos) = self.order.iter().position(|k| k == key) {
            let k = self.order.remove(pos);
            self.order.push(k);
        }
    }
}

impl<K: Eq + Hash + Clone, V> AlgorithmTrait for LruCache<K, V> {
    fn name(&self) -> &'static str {
        "lru_cache"
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

impl<K: Eq + Hash + Clone, V> CacheAlgorithmTrait<K, V> for LruCache<K, V> {
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
    fn test_put_and_get() {
        let mut cache: LruCache<&str, i32> = LruCache::new(2);
        cache.put("a", 1);
        cache.put("b", 2);
        assert_eq!(cache.get(&"a"), Some(&1));
        assert_eq!(cache.get(&"b"), Some(&2));
    }

    #[test]
    fn test_evict_lru() {
        let mut cache: LruCache<&str, i32> = LruCache::new(2);
        cache.put("a", 1);
        cache.put("b", 2);
        cache.get(&"a");
        cache.put("c", 3);
        assert!(cache.contains(&"a"));
        assert!(!cache.contains(&"b"));
        assert!(cache.contains(&"c"));
    }

    #[test]
    fn test_update_existing() {
        let mut cache: LruCache<&str, i32> = LruCache::new(2);
        cache.put("a", 1);
        cache.put("a", 10);
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.peek(&"a"), Some(&10));
    }

    #[test]
    fn test_remove() {
        let mut cache: LruCache<&str, i32> = LruCache::new(2);
        cache.put("a", 1);
        assert_eq!(cache.remove(&"a"), Some(1));
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.remove(&"a"), None);
    }

    #[test]
    fn test_clear() {
        let mut cache: LruCache<i32, i32> = LruCache::new(3);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.clear();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_capacity_one() {
        let mut cache: LruCache<i32, i32> = LruCache::new(1);
        cache.put(1, 10);
        cache.put(2, 20);
        assert_eq!(cache.len(), 1);
        assert!(!cache.contains(&1));
        assert!(cache.contains(&2));
    }

    #[test]
    fn test_peek_does_not_update_order() {
        let mut cache: LruCache<&str, i32> = LruCache::new(2);
        cache.put("a", 1);
        cache.put("b", 2);
        cache.peek(&"a");
        cache.put("c", 3);
        assert!(!cache.contains(&"a"));
    }

    #[test]
    fn test_get_mut() {
        let mut cache: LruCache<&str, i32> = LruCache::new(2);
        cache.put("a", 1);
        if let Some(v) = cache.get_mut(&"a") {
            *v += 10;
        }
        assert_eq!(cache.peek(&"a"), Some(&11));
    }

    #[test]
    fn test_iter() {
        let mut cache: LruCache<i32, i32> = LruCache::new(3);
        cache.put(1, 10);
        cache.put(2, 20);
        let collected: Vec<(i32, i32)> = cache.iter().map(|(k, v)| (*k, *v)).collect();
        assert_eq!(collected.len(), 2);
    }
}
