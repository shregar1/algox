use crate::abstraction::AlgorithmTrait;
use super::abstraction::ShardingAlgorithmTrait;
use crate::hashing::fnv1a_64;
use std::collections::BTreeMap;

/// Consistent Hashing implementation with virtual nodes (vnodes) for uniform load distribution.
pub struct ConsistentHash {
    vnodes_per_node: usize,
    ring: BTreeMap<u64, String>,
    nodes: Vec<String>,
}

impl ConsistentHash {
    pub fn new(vnodes_per_node: usize) -> Self {
        Self {
            vnodes_per_node: vnodes_per_node.max(1),
            ring: BTreeMap::new(),
            nodes: Vec::new(),
        }
    }

    /// Adds a node / shard to the hash ring.
    pub fn add_node(&mut self, node: &str) {
        if self.nodes.contains(&node.to_string()) {
            return;
        }
        self.nodes.push(node.to_string());
        for i in 0..self.vnodes_per_node {
            let vnode_key = format!("{}-vnode-{}", node, i);
            let hash = fnv1a_64(vnode_key.as_bytes());
            self.ring.insert(hash, node.to_string());
        }
    }

    /// Removes a node / shard from the hash ring.
    pub fn remove_node(&mut self, node: &str) {
        if let Some(pos) = self.nodes.iter().position(|n| n == node) {
            self.nodes.remove(pos);
            for i in 0..self.vnodes_per_node {
                let vnode_key = format!("{}-vnode-{}", node, i);
                let hash = fnv1a_64(vnode_key.as_bytes());
                self.ring.remove(&hash);
            }
        }
    }

    /// Gets the responsible node for a given key string.
    pub fn get_node(&self, key: &str) -> Option<String> {
        if self.ring.is_empty() {
            return None;
        }
        let hash = fnv1a_64(key.as_bytes());
        // Find first entry in ring >= hash, or wrap around to first entry
        if let Some((_, node)) = self.ring.range(hash..).next() {
            Some(node.clone())
        } else {
            self.ring.values().next().cloned()
        }
    }
}

impl AlgorithmTrait for ConsistentHash {
    fn name(&self) -> &'static str {
        "consistent_hash"
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn clear(&mut self) {
        self.ring.clear();
        self.nodes.clear();
    }
}

impl ShardingAlgorithmTrait for ConsistentHash {
    fn get_shard(&self, key: &str) -> Option<String> {
        self.get_node(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consistent_hashing() {
        let mut ch = ConsistentHash::new(10);
        ch.add_node("shard-1");
        ch.add_node("shard-2");
        ch.add_node("shard-3");

        let node1 = ch.get_node("user_1001").unwrap();
        let node2 = ch.get_node("user_1002").unwrap();
        assert!(["shard-1", "shard-2", "shard-3"].contains(&node1.as_str()));
        assert!(["shard-1", "shard-2", "shard-3"].contains(&node2.as_str()));

        // Removing a node redistributes keys
        ch.remove_node("shard-2");
        let new_node = ch.get_node("user_1001").unwrap();
        assert_ne!(new_node, "shard-2");
    }
}
