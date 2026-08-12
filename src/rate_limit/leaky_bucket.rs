use crate::abstraction::AlgorithmTrait;
use super::abstraction::RateLimitAlgorithmTrait;
use std::collections::HashMap;
use std::time::Instant;

struct LeakyState {
    water_level: f64,
    last_leak: Instant,
}

pub struct LeakyBucket {
    capacity: f64,
    leak_rate: f64, // units leaked per second
    buckets: HashMap<String, LeakyState>,
}

impl LeakyBucket {
    pub fn new(capacity: f64, leak_rate: f64) -> Self {
        Self {
            capacity,
            leak_rate,
            buckets: HashMap::new(),
        }
    }
}

impl AlgorithmTrait for LeakyBucket {
    fn name(&self) -> &'static str {
        "leaky_bucket"
    }

    fn len(&self) -> usize {
        self.buckets.len()
    }

    fn clear(&mut self) {
        self.buckets.clear();
    }
}

impl RateLimitAlgorithmTrait for LeakyBucket {
    fn check_and_consume(&mut self, key: &str, cost: u64) -> bool {
        let now = Instant::now();
        let leak_rate = self.leak_rate;

        let state = self.buckets.entry(key.to_string()).or_insert_with(|| LeakyState {
            water_level: 0.0,
            last_leak: now,
        });

        let elapsed = now.duration_since(state.last_leak).as_secs_f64();
        state.water_level = (state.water_level - elapsed * leak_rate).max(0.0);
        state.last_leak = now;

        if state.water_level + cost as f64 <= self.capacity {
            state.water_level += cost as f64;
            true
        } else {
            false
        }
    }

    fn reset_key(&mut self, key: &str) {
        self.buckets.remove(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaky_bucket() {
        let mut lb = LeakyBucket::new(10.0, 1.0);
        assert!(lb.check_and_consume("user1", 5));
        assert!(lb.check_and_consume("user1", 5));
        assert!(!lb.check_and_consume("user1", 1));
    }
}
