use crate::abstraction::AlgorithmTrait;
use super::abstraction::ShardingAlgorithmTrait;
use std::collections::BTreeMap;

/// Range-Based Key Partitioning / Sharding router.
pub struct RangeSharder<K: Ord + Clone> {
    ranges: BTreeMap<K, String>,
}

impl<K: Ord + Clone> RangeSharder<K> {
    pub fn new() -> Self {
        Self { ranges: BTreeMap::new() }
    }

    /// Adds a shard boundary range. `max_key` specifies the upper bound for `shard_id`.
    pub fn add_range(&mut self, upper_bound: K, shard_id: &str) {
        self.ranges.insert(upper_bound, shard_id.to_string());
    }

    /// Routes `key` to its responsible range shard.
    pub fn get_shard_by_key(&self, key: &K) -> Option<String> {
        self.ranges.range(key..).next().map(|(_, shard)| shard.clone())
    }
}

impl AlgorithmTrait for RangeSharder<i64> {
    fn name(&self) -> &'static str {
        "range_sharding"
    }

    fn len(&self) -> usize {
        self.ranges.len()
    }

    fn clear(&mut self) {
        self.ranges.clear();
    }
}

impl ShardingAlgorithmTrait for RangeSharder<i64> {
    fn get_shard(&self, key: &str) -> Option<String> {
        if let Ok(val) = key.parse::<i64>() {
            self.get_shard_by_key(&val)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_sharding() {
        let mut router = RangeSharder::new();
        router.add_range(1000, "shard-alpha");
        router.add_range(5000, "shard-beta");
        router.add_range(10000, "shard-gamma");

        assert_eq!(router.get_shard_by_key(&500).unwrap(), "shard-alpha");
        assert_eq!(router.get_shard_by_key(&3500).unwrap(), "shard-beta");
        assert_eq!(router.get_shard_by_key(&7500).unwrap(), "shard-gamma");
    }
}
