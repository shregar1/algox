use crate::abstraction::AlgorithmTrait;
use super::abstraction::GraphAlgorithmTrait;
use std::collections::VecDeque;

pub struct TopologicalSort;

impl TopologicalSort {
    pub fn sort(nodes: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
        let mut in_degree = vec![0usize; nodes];
        let mut adj = vec![Vec::new(); nodes];

        for &(u, v) in edges {
            if u < nodes && v < nodes {
                adj[u].push(v);
                in_degree[v] += 1;
            }
        }

        let mut queue = VecDeque::new();
        for i in 0..nodes {
            if in_degree[i] == 0 {
                queue.push_back(i);
            }
        }

        let mut result = Vec::with_capacity(nodes);
        while let Some(u) = queue.pop_front() {
            result.push(u);
            for &v in &adj[u] {
                in_degree[v] -= 1;
                if in_degree[v] == 0 {
                    queue.push_back(v);
                }
            }
        }

        if result.len() == nodes {
            Some(result)
        } else {
            None // Cycle detected
        }
    }
}

impl AlgorithmTrait for TopologicalSort {
    fn name(&self) -> &'static str {
        "topological_sort"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl GraphAlgorithmTrait for TopologicalSort {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topological_sort() {
        let edges = vec![(5, 2), (5, 0), (4, 0), (4, 1), (2, 3), (3, 1)];
        let res = TopologicalSort::sort(6, &edges);
        assert!(res.is_some());
        let order = res.unwrap();
        assert_eq!(order.len(), 6);
    }
}
