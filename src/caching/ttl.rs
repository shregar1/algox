use crate::abstraction::AlgorithmTrait;
use super::abstraction::CacheAlgorithmTrait;
use std::collections::HashMap;
use std::hash::Hash;
use std::num::NonZeroUsize;
use std::time::{Duration, Instant};

struct TtlEntry<V> {
    value: V,
    expires_at: Instant,
}

pub struct TtlCache<K, V> {
    capacity: NonZeroUsize,
    default_ttl: Duration,
    map: HashMap<K, TtlEntry<V>>,
}

impl<K: Eq + Hash + Clone, V> TtlCache<K, V> {
    pub fn new(capacity: usize, default_ttl: Duration) -> Self {
        let capacity = NonZeroUsize::new(capacity.max(1)).expect("capacity must be non-zero");
        Self {
            capacity,
            default_ttl,
            map: HashMap::with_capacity(capacity.get()),
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity.get()
    }

    pub fn len(&self) -> usize {
        let now = Instant::now();
        self.map.values().filter(|e| e.expires_at > now).count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn contains(&self, key: &K) -> bool {
        if let Some(entry) = self.map.get(key) {
            if Instant::now() < entry.expires_at {
                return true;
            }
        }
        false
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        let now = Instant::now();
        let is_expired = self.map.get(key).map_or(false, |entry| now >= entry.expires_at);
        if is_expired {
            self.map.remove(key);
            None
        } else {
            self.map.get(key).map(|e| &e.value)
        }
    }

    pub fn peek(&self, key: &K) -> Option<&V> {
        if let Some(entry) = self.map.get(key) {
            if Instant::now() < entry.expires_at {
                return Some(&entry.value);
            }
        }
        None
    }

    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        self.put_with_ttl(key, value, self.default_ttl)
    }

    pub fn put_with_ttl(&mut self, key: K, value: V, ttl: Duration) -> Option<V> {
        self.purge_expired();

        let expires_at = Instant::now() + ttl;
        let old = self.map.insert(key.clone(), TtlEntry { value, expires_at });

        if self.map.len() > self.capacity.get() {
            // Evict the entry expiring earliest
            if let Some(earliest_key) = self
                .map
                .iter()
                .min_by_key(|(_, entry)| entry.expires_at)
                .map(|(k, _)| k.clone())
            {
                self.map.remove(&earliest_key);
            }
        }

        old.map(|e| e.value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key).and_then(|entry| {
            if Instant::now() < entry.expires_at {
                Some(entry.value)
            } else {
                None
            }
        })
    }

    pub fn purge_expired(&mut self) {
        let now = Instant::now();
        self.map.retain(|_, entry| entry.expires_at > now);
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}

impl<K: Eq + Hash + Clone, V> AlgorithmTrait for TtlCache<K, V> {
    fn name(&self) -> &'static str {
        "ttl_cache"
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

impl<K: Eq + Hash + Clone, V> CacheAlgorithmTrait<K, V> for TtlCache<K, V> {
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
    use std::thread::sleep;

    #[test]
    fn test_ttl_expiration() {
        let mut cache: TtlCache<&str, i32> = TtlCache::new(5, Duration::from_millis(50));
        cache.put("a", 100);
        assert_eq!(cache.get(&"a"), Some(&100));

        sleep(Duration::from_millis(60));
        assert_eq!(cache.get(&"a"), None);
    }
}
