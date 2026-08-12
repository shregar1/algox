use crate::abstraction::AlgorithmTrait;
use super::abstraction::ShardingAlgorithmTrait;
use crate::hashing::fnv1a_64;

/// Rendezvous Hashing (Highest Random Weight / HRW) algorithm.
pub struct RendezvousSharding {
    nodes: Vec<String>,
}

impl RendezvousSharding {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: &str) {
        if !self.nodes.contains(&node.to_string()) {
            self.nodes.push(node.to_string());
        }
    }

    pub fn remove_node(&mut self, node: &str) {
        if let Some(pos) = self.nodes.iter().position(|n| n == node) {
            self.nodes.remove(pos);
        }
    }

    pub fn get_node(&self, key: &str) -> Option<String> {
        if self.nodes.is_empty() {
            return None;
        }

        let mut max_weight = 0u64;
        let mut best_node = None;

        for node in &self.nodes {
            let combined = format!("{}-{}", key, node);
            let weight = fnv1a_64(combined.as_bytes());
            if weight >= max_weight {
                max_weight = weight;
                best_node = Some(node.clone());
            }
        }

        best_node
    }
}

impl AlgorithmTrait for RendezvousSharding {
    fn name(&self) -> &'static str {
        "rendezvous_sharding"
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn clear(&mut self) {
        self.nodes.clear();
    }
}

impl ShardingAlgorithmTrait for RendezvousSharding {
    fn get_shard(&self, key: &str) -> Option<String> {
        self.get_node(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rendezvous_sharding() {
        let mut rh = RendezvousSharding::new();
        rh.add_node("node-a");
        rh.add_node("node-b");
        rh.add_node("node-c");

        let target = rh.get_node("session_key_42").unwrap();
        assert!(["node-a", "node-b", "node-c"].contains(&target.as_str()));
    }
}
