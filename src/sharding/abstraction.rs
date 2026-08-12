use crate::abstraction::AlgorithmTrait;

/// Trait interface for sharding and partitioning algorithms.
pub trait ShardingAlgorithmTrait: AlgorithmTrait {
    /// Returns the target shard identifier for a given key string.
    fn get_shard(&self, key: &str) -> Option<String>;
}
