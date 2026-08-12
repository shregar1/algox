use crate::abstraction::AlgorithmTrait;
use super::abstraction::LoadBalancingAlgorithmTrait;
use std::collections::HashMap;

pub struct LeastConnections {
    active_conns: HashMap<usize, usize>,
}

impl LeastConnections {
    pub fn new() -> Self {
        Self {
            active_conns: HashMap::new(),
        }
    }

    pub fn increment_conn(&mut self, target_idx: usize) {
        *self.active_conns.entry(target_idx).or_insert(0) += 1;
    }

    pub fn decrement_conn(&mut self, target_idx: usize) {
        if let Some(count) = self.active_conns.get_mut(&target_idx) {
            *count = count.saturating_sub(1);
        }
    }
}

impl Default for LeastConnections {
    fn default() -> Self {
        Self::new()
    }
}

impl AlgorithmTrait for LeastConnections {
    fn name(&self) -> &'static str {
        "least_connections"
    }

    fn len(&self) -> usize {
        self.active_conns.len()
    }

    fn clear(&mut self) {
        self.active_conns.clear();
    }
}

impl<T> LoadBalancingAlgorithmTrait<T> for LeastConnections {
    fn select<'a>(&mut self, targets: &'a [T]) -> Option<&'a T> {
        if targets.is_empty() {
            return None;
        }

        let mut min_idx = 0;
        let mut min_conns = usize::MAX;

        for (idx, _) in targets.iter().enumerate() {
            let conns = self.active_conns.get(&idx).copied().unwrap_or(0);
            if conns < min_conns {
                min_conns = conns;
                min_idx = idx;
            }
        }

        Some(&targets[min_idx])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_least_connections() {
        let mut lc = LeastConnections::default();
        assert_eq!(lc.name(), "least_connections");

        let empty: Vec<&str> = vec![];
        assert_eq!(lc.select(&empty), None);

        let targets = vec!["node1", "node2", "node3"];

        lc.increment_conn(0);
        lc.increment_conn(0);
        lc.increment_conn(1);
        assert_eq!(lc.len(), 2);

        assert_eq!(lc.select(&targets), Some(&"node3"));

        lc.decrement_conn(0);
        lc.decrement_conn(0);
        lc.decrement_conn(99); // Decrementing non-existent target

        lc.clear();
        assert_eq!(lc.len(), 0);
    }
}
