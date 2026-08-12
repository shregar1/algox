use crate::abstraction::AlgorithmTrait;
use super::abstraction::RateLimitAlgorithmTrait;
use std::collections::HashMap;
use std::time::Instant;

struct Bucket {
    tokens: f64,
    last_refill: Instant,
}

pub struct TokenBucket {
    capacity: f64,
    refill_rate: f64, // tokens per second
    buckets: HashMap<String, Bucket>,
}

impl TokenBucket {
    pub fn new(capacity: f64, refill_rate: f64) -> Self {
        Self {
            capacity,
            refill_rate,
            buckets: HashMap::new(),
        }
    }
}

impl AlgorithmTrait for TokenBucket {
    fn name(&self) -> &'static str {
        "token_bucket"
    }

    fn len(&self) -> usize {
        self.buckets.len()
    }

    fn clear(&mut self) {
        self.buckets.clear();
    }
}

impl RateLimitAlgorithmTrait for TokenBucket {
    fn check_and_consume(&mut self, key: &str, cost: u64) -> bool {
        let now = Instant::now();
        let capacity = self.capacity;
        let refill_rate = self.refill_rate;

        let bucket = self.buckets.entry(key.to_string()).or_insert_with(|| Bucket {
            tokens: capacity,
            last_refill: now,
        });

        let elapsed = now.duration_since(bucket.last_refill).as_secs_f64();
        bucket.tokens = (bucket.tokens + elapsed * refill_rate).min(capacity);
        bucket.last_refill = now;

        let cost_f = cost as f64;
        if bucket.tokens >= cost_f {
            bucket.tokens -= cost_f;
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
    fn test_token_bucket() {
        let mut tb = TokenBucket::new(10.0, 1.0);
        assert_eq!(tb.name(), "token_bucket");
        assert_eq!(tb.len(), 0);

        assert!(tb.check_and_consume("user1", 5));
        assert_eq!(tb.len(), 1);
        assert!(tb.check_and_consume("user1", 5));
        assert!(!tb.check_and_consume("user1", 1));

        tb.reset_key("user1");
        assert_eq!(tb.len(), 0);
        assert!(tb.check_and_consume("user1", 5));

        tb.clear();
        assert_eq!(tb.len(), 0);
    }
}
