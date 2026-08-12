use crate::abstraction::AlgorithmTrait;
use super::abstraction::ShardingAlgorithmTrait;
use crate::hashing::fnv1a_64;

/// Composite Key Sharder.
pub struct CompositeSharding {
    delimiter: char,
    primary_index: usize,
    num_shards: usize,
}

impl CompositeSharding {
    pub fn new(delimiter: char, primary_index: usize, num_shards: usize) -> Self {
        Self {
            delimiter,
            primary_index,
            num_shards: num_shards.max(1),
        }
    }

    pub fn get_shard_index(&self, composite_key: &str) -> usize {
        let parts: Vec<&str> = composite_key.split(self.delimiter).collect();
        let target_part = if self.primary_index < parts.len() {
            parts[self.primary_index]
        } else {
            composite_key
        };

        let hash = fnv1a_64(target_part.as_bytes());
        (hash as usize) % self.num_shards
    }
}

impl AlgorithmTrait for CompositeSharding {
    fn name(&self) -> &'static str {
        "composite_sharding"
    }

    fn len(&self) -> usize {
        self.num_shards
    }

    fn clear(&mut self) {}
}

impl ShardingAlgorithmTrait for CompositeSharding {
    fn get_shard(&self, key: &str) -> Option<String> {
        let idx = self.get_shard_index(key);
        Some(format!("shard-{}", idx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composite_sharding() {
        let sharder = CompositeSharding::new(':', 0, 16);

        let idx1 = sharder.get_shard_index("acme:user_1:order_99");
        let idx2 = sharder.get_shard_index("acme:user_2:order_100");
        let idx3 = sharder.get_shard_index("acme:invoice_5");

        assert_eq!(idx1, idx2);
        assert_eq!(idx2, idx3);
    }
}
