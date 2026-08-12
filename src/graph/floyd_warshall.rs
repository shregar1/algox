use crate::abstraction::AlgorithmTrait;
use super::abstraction::GraphAlgorithmTrait;

/// Floyd-Warshall algorithm for All-Pairs Shortest Paths in O(V³).
pub struct FloydWarshall;

impl FloydWarshall {
    /// Computes all-pairs shortest paths. `dist[u][v]` is initial weight from u to v,
    /// or `i64::MAX` if no edge exists.
    /// Returns 2D matrix of shortest path distances.
    pub fn solve(nodes: usize, mut dist: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
        for i in 0..nodes {
            dist[i][i] = 0;
        }

        for k in 0..nodes {
            for i in 0..nodes {
                for j in 0..nodes {
                    if dist[i][k] != i64::MAX && dist[k][j] != i64::MAX {
                        if dist[i][k] + dist[k][j] < dist[i][j] {
                            dist[i][j] = dist[i][k] + dist[k][j];
                        }
                    }
                }
            }
        }
        dist
    }
}

impl AlgorithmTrait for FloydWarshall {
    fn name(&self) -> &'static str {
        "floyd_warshall"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl GraphAlgorithmTrait for FloydWarshall {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floyd_warshall() {
        let inf = i64::MAX;
        let dist = vec![
            vec![0, 5, inf, 10],
            vec![inf, 0, 3, inf],
            vec![inf, inf, 0, 1],
            vec![inf, inf, inf, 0],
        ];

        let result = FloydWarshall::solve(4, dist);
        assert_eq!(result[0][2], 8); // 0 -> 1 -> 2
        assert_eq!(result[0][3], 9); // 0 -> 1 -> 2 -> 3
    }
}
