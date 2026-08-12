use crate::abstraction::AlgorithmTrait;
use super::abstraction::ShardingAlgorithmTrait;
use std::collections::HashMap;

/// Directory / Map-Based Dynamic Partition Router.
pub struct DirectorySharding {
    directory: HashMap<String, String>,
}

impl DirectorySharding {
    pub fn new() -> Self {
        Self { directory: HashMap::new() }
    }

    pub fn bind(&mut self, entity_key: &str, shard_id: &str) {
        self.directory.insert(entity_key.to_string(), shard_id.to_string());
    }

    pub fn unbind(&mut self, entity_key: &str) -> Option<String> {
        self.directory.remove(entity_key)
    }

    pub fn lookup(&self, entity_key: &str) -> Option<String> {
        self.directory.get(entity_key).cloned()
    }
}

impl AlgorithmTrait for DirectorySharding {
    fn name(&self) -> &'static str {
        "directory_sharding"
    }

    fn len(&self) -> usize {
        self.directory.len()
    }

    fn clear(&mut self) {
        self.directory.clear();
    }
}

impl ShardingAlgorithmTrait for DirectorySharding {
    fn get_shard(&self, key: &str) -> Option<String> {
        self.lookup(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directory_sharding() {
        let mut router = DirectorySharding::new();
        router.bind("account_vip_99", "shard-nvme-ultra");
        router.bind("account_std_01", "shard-hdd-1");

        assert_eq!(router.lookup("account_vip_99").unwrap(), "shard-nvme-ultra");
        assert_eq!(router.lookup("account_std_01").unwrap(), "shard-hdd-1");
        assert!(router.lookup("account_unknown").is_none());
    }
}
