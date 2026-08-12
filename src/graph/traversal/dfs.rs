use crate::abstraction::AlgorithmTrait;
use super::super::abstraction::GraphAlgorithmTrait;

pub struct DFS;

impl DFS {
    pub fn traverse(nodes: usize, adj: &[Vec<usize>], start: usize) -> Vec<usize> {
        let mut visited = vec![false; nodes];
        let mut order = Vec::with_capacity(nodes);
        if start < nodes {
            Self::traverse_with_visited(start, adj, &mut visited, &mut order);
        }
        order
    }

    pub fn traverse_with_visited(
        u: usize,
        adj: &[Vec<usize>],
        visited: &mut [bool],
        order: &mut Vec<usize>,
    ) {
        visited[u] = true;
        order.push(u);
        for &v in &adj[u] {
            if !visited[v] {
                Self::traverse_with_visited(v, adj, visited, order);
            }
        }
    }

    pub fn dfs_lowlink<F>(
        u: usize,
        p: usize,
        adj: &[Vec<usize>],
        visited: &mut [bool],
        tin: &mut [usize],
        low: &mut [usize],
        timer: &mut usize,
        on_bridge: &mut F,
    ) where
        F: FnMut(usize, usize),
    {
        visited[u] = true;
        *timer += 1;
        tin[u] = *timer;
        low[u] = *timer;

        for &v in &adj[u] {
            if v == p {
                continue;
            }
            if visited[v] {
                low[u] = low[u].min(tin[v]);
            } else {
                Self::dfs_lowlink(v, u, adj, visited, tin, low, timer, on_bridge);
                low[u] = low[u].min(low[v]);
                if low[v] > tin[u] {
                    on_bridge(u, v);
                }
            }
        }
    }
}

impl AlgorithmTrait for DFS {
    fn name(&self) -> &'static str {
        "dfs"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl GraphAlgorithmTrait for DFS {}

impl super::abstraction::GraphTraversalAlgorithmTrait for DFS {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs() {
        let adj = vec![vec![1, 2], vec![2], vec![0, 3], vec![3]];
        let dfs_order = DFS::traverse(4, &adj, 2);
        assert_eq!(dfs_order, vec![2, 0, 1, 3]);
    }
}
