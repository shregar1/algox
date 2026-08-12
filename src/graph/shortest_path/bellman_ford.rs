use crate::abstraction::AlgorithmTrait;
use super::super::abstraction::GraphAlgorithmTrait;
use super::abstraction::ShortestPathAlgorithmTrait;

pub struct BellmanFord;

impl BellmanFord {
    pub fn shortest_path(
        nodes: usize,
        edges: &[(usize, usize, i64)],
        start: usize,
    ) -> Option<Vec<i64>> {
        let mut dist = vec![i64::MAX; nodes];
        dist[start] = 0;

        for _ in 0..nodes - 1 {
            for &(u, v, w) in edges {
                if dist[u] != i64::MAX && dist[u] + w < dist[v] {
                    dist[v] = dist[u] + w;
                }
            }
        }

        for &(u, v, w) in edges {
            if dist[u] != i64::MAX && dist[u] + w < dist[v] {
                return None; // Negative cycle detected
            }
        }

        Some(dist)
    }
}

impl AlgorithmTrait for BellmanFord {
    fn name(&self) -> &'static str {
        "bellman_ford"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl GraphAlgorithmTrait for BellmanFord {}

impl ShortestPathAlgorithmTrait for BellmanFord {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bellman_ford() {
        let edges = vec![
            (0, 1, 4),
            (0, 2, 2),
            (1, 2, 3),
            (2, 1, 1),
            (1, 3, 2),
            (2, 3, 4),
        ];
        let dist = BellmanFord::shortest_path(4, &edges, 0);
        assert!(dist.is_some());
        let res = dist.unwrap();
        assert_eq!(res[3], 5);
    }
}
