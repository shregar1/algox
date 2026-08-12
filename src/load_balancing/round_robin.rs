use crate::abstraction::AlgorithmTrait;
use super::abstraction::LoadBalancingAlgorithmTrait;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct RoundRobin {
    index: AtomicUsize,
}

impl RoundRobin {
    pub fn new() -> Self {
        Self {
            index: AtomicUsize::new(0),
        }
    }
}

impl Default for RoundRobin {
    fn default() -> Self {
        Self::new()
    }
}

impl AlgorithmTrait for RoundRobin {
    fn name(&self) -> &'static str {
        "round_robin"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {
        self.index.store(0, Ordering::Relaxed);
    }
}

impl<T> LoadBalancingAlgorithmTrait<T> for RoundRobin {
    fn select<'a>(&mut self, targets: &'a [T]) -> Option<&'a T> {
        if targets.is_empty() {
            return None;
        }
        let current = self.index.fetch_add(1, Ordering::Relaxed);
        Some(&targets[current % targets.len()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_robin() {
        let mut rr = RoundRobin::new();
        let targets = vec!["server1", "server2", "server3"];
        assert_eq!(rr.select(&targets), Some(&"server1"));
        assert_eq!(rr.select(&targets), Some(&"server2"));
        assert_eq!(rr.select(&targets), Some(&"server3"));
        assert_eq!(rr.select(&targets), Some(&"server1"));
    }
}
