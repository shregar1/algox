use crate::abstraction::AlgorithmTrait;
use super::abstraction::ShardingAlgorithmTrait;
use std::collections::HashMap;

/// Categorical List Partitioning Router.
pub struct ListSharding {
    partition_map: HashMap<String, String>, // category -> shard_id
    fallback_shard: Option<String>,
}

impl ListSharding {
    pub fn new() -> Self {
        Self {
            partition_map: HashMap::new(),
            fallback_shard: None,
        }
    }

    pub fn set_fallback(&mut self, shard_id: &str) {
        self.fallback_shard = Some(shard_id.to_string());
    }

    pub fn register_partition(&mut self, categories: &[&str], shard_id: &str) {
        for category in categories {
            self.partition_map.insert(category.to_string(), shard_id.to_string());
        }
    }

    pub fn get_shard_for_category(&self, category: &str) -> Option<String> {
        self.partition_map
            .get(category)
            .cloned()
            .or_else(|| self.fallback_shard.clone())
    }
}

impl AlgorithmTrait for ListSharding {
    fn name(&self) -> &'static str {
        "list_sharding"
    }

    fn len(&self) -> usize {
        self.partition_map.len()
    }

    fn clear(&mut self) {
        self.partition_map.clear();
        self.fallback_shard = None;
    }
}

impl ShardingAlgorithmTrait for ListSharding {
    fn get_shard(&self, key: &str) -> Option<String> {
        self.get_shard_for_category(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_sharding() {
        let mut router = ListSharding::new();
        router.set_fallback("shard-global");
        router.register_partition(&["US", "CA", "MX"], "shard-na");
        router.register_partition(&["UK", "DE", "FR"], "shard-eu");

        assert_eq!(router.get_shard_for_category("US").unwrap(), "shard-na");
        assert_eq!(router.get_shard_for_category("DE").unwrap(), "shard-eu");
        assert_eq!(router.get_shard_for_category("JP").unwrap(), "shard-global");
    }
}
