use crate::abstraction::AlgorithmTrait;
use super::abstraction::ShardingAlgorithmTrait;
use crate::hashing::fnv1a_64;

/// Google's Jump Consistent Hash algorithm.
/// Fast, zero-memory consistent hashing that maps a key to a bucket in 0..num_buckets in O(ln N) time.
pub struct JumpConsistentSharding;

impl JumpConsistentSharding {
    /// Computes the bucket index in `0..num_buckets` for a 64-bit key.
    pub fn hash(mut key: u64, num_buckets: usize) -> usize {
        if num_buckets <= 1 {
            return 0;
        }

        let mut b: i64 = -1;
        let mut j: i64 = 0;

        while j < num_buckets as i64 {
            b = j;
            key = key.wrapping_mul(2862933555777941757).wrapping_add(1);
            let f = ((1u64 << 31) as f64) / (((key >> 33) + 1) as f64);
            j = ((b + 1) as f64 * f) as i64;
        }

        b as usize
    }

    /// Computes bucket for a string key.
    pub fn hash_key(key: &str, num_buckets: usize) -> usize {
        let key_64 = fnv1a_64(key.as_bytes());
        Self::hash(key_64, num_buckets)
    }
}

impl AlgorithmTrait for JumpConsistentSharding {
    fn name(&self) -> &'static str {
        "jump_consistent_sharding"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl ShardingAlgorithmTrait for JumpConsistentSharding {
    fn get_shard(&self, key: &str) -> Option<String> {
        Some(format!("shard-{}", Self::hash_key(key, 10)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jump_consistent_sharding() {
        let b1 = JumpConsistentSharding::hash(12345, 10);
        let b2 = JumpConsistentSharding::hash(12345, 10);
        assert_eq!(b1, b2);
        assert!(b1 < 10);

        let key_b = JumpConsistentSharding::hash_key("user_tenant_99", 100);
        assert!(key_b < 100);
    }
}
