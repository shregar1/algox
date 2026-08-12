use crate::abstraction::AlgorithmTrait;
use super::abstraction::CacheAlgorithmTrait;
use std::collections::HashMap;
use std::hash::Hash;
use std::num::NonZeroUsize;

/// Adaptive Replacement Cache (ARC)
pub struct ArcCache<K, V> {
    capacity: NonZeroUsize,
    t1: Vec<K>, // Recency list
    t2: Vec<K>, // Frequency list
    b1: Vec<K>, // Recency ghost list
    b2: Vec<K>, // Frequency ghost list
    p: usize,   // Target size for T1
    map: HashMap<K, V>,
}

impl<K: Eq + Hash + Clone, V> ArcCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        let capacity = NonZeroUsize::new(capacity.max(1)).expect("capacity must be non-zero");
        let cap = capacity.get();
        Self {
            capacity,
            t1: Vec::with_capacity(cap),
            t2: Vec::with_capacity(cap),
            b1: Vec::with_capacity(cap),
            b2: Vec::with_capacity(cap),
            p: 0,
            map: HashMap::with_capacity(cap),
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
        if self.map.contains_key(key) {
            self.touch(key);
            self.map.get(key)
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
            if self.t1.len() + self.b1.len() == self.capacity.get() {
                if self.t1.len() < self.capacity.get() {
                    self.b1.remove(0);
                    self.replace(&key);
                } else {
                    let victim = self.t1.remove(0);
                    self.map.remove(&victim);
                }
            } else if self.t1.len() + self.b1.len() < self.capacity.get()
                && self.t1.len() + self.t2.len() + self.b1.len() + self.b2.len()
                    >= self.capacity.get()
            {
                if self.t1.len() + self.t2.len() + self.b1.len() + self.b2.len()
                    == 2 * self.capacity.get()
                {
                    self.b2.remove(0);
                }
                self.replace(&key);
            }
            self.t1.push(key);
        }
        old
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let val = self.map.remove(key);
        if val.is_some() {
            self.t1.retain(|k| k != key);
            self.t2.retain(|k| k != key);
            self.b1.retain(|k| k != key);
            self.b2.retain(|k| k != key);
        }
        val
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.t1.clear();
        self.t2.clear();
        self.b1.clear();
        self.b2.clear();
        self.p = 0;
    }

    fn touch(&mut self, key: &K) {
        if let Some(pos) = self.t1.iter().position(|k| k == key) {
            let k = self.t1.remove(pos);
            self.t2.push(k);
        } else if let Some(pos) = self.t2.iter().position(|k| k == key) {
            let k = self.t2.remove(pos);
            self.t2.push(k);
        }
    }

    fn replace(&mut self, _key: &K) {
        if !self.t1.is_empty()
            && (self.t1.len() > self.p || (!self.b2.is_empty() && self.t1.len() == self.p))
        {
            let victim = self.t1.remove(0);
            self.b1.push(victim.clone());
            self.map.remove(&victim);
        } else if !self.t2.is_empty() {
            let victim = self.t2.remove(0);
            self.b2.push(victim.clone());
            self.map.remove(&victim);
        }
    }
}

impl<K: Eq + Hash + Clone, V> AlgorithmTrait for ArcCache<K, V> {
    fn name(&self) -> &'static str {
        "arc_cache"
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

impl<K: Eq + Hash + Clone, V> CacheAlgorithmTrait<K, V> for ArcCache<K, V> {
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
    fn test_arc() {
        let mut cache: ArcCache<&str, i32> = ArcCache::new(2);
        cache.put("a", 1);
        cache.put("b", 2);
        assert_eq!(cache.get(&"a"), Some(&1));
        cache.put("c", 3);
        assert_eq!(cache.len(), 2);
    }
}
