use crate::abstraction::AlgorithmTrait;
use super::super::abstraction::GraphAlgorithmTrait;
use super::abstraction::GraphTraversalAlgorithmTrait;
use std::collections::VecDeque;

pub struct BFS;

impl BFS {
    pub fn traverse(nodes: usize, adj: &[Vec<usize>], start: usize) -> Vec<usize> {
        let mut visited = vec![false; nodes];
        let mut order = Vec::with_capacity(nodes);
        let mut queue = VecDeque::new();

        if start < nodes {
            visited[start] = true;
            queue.push_back(start);
        }

        while let Some(u) = queue.pop_front() {
            order.push(u);
            for &v in &adj[u] {
                if !visited[v] {
                    visited[v] = true;
                    queue.push_back(v);
                }
            }
        }

        order
    }
}

impl AlgorithmTrait for BFS {
    fn name(&self) -> &'static str {
        "bfs"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl GraphAlgorithmTrait for BFS {}

impl GraphTraversalAlgorithmTrait for BFS {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        let adj = vec![vec![1, 2], vec![2], vec![0, 3], vec![3]];
        let bfs_order = BFS::traverse(4, &adj, 2);
        assert_eq!(bfs_order, vec![2, 0, 3, 1]);
    }
}
