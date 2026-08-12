use crate::abstraction::AlgorithmTrait;
use super::abstraction::GraphAlgorithmTrait;
use super::traversal::DFS;

pub struct Bridge;

impl Bridge {
    pub fn find(nodes: usize, adj: &[Vec<usize>]) -> Vec<(usize, usize)> {
        let mut visited = vec![false; nodes];
        let mut tin = vec![0usize; nodes];
        let mut low = vec![0usize; nodes];
        let mut timer = 0;
        let mut bridges = Vec::new();

        for i in 0..nodes {
            if !visited[i] {
                DFS::dfs_lowlink(
                    i,
                    usize::MAX,
                    adj,
                    &mut visited,
                    &mut tin,
                    &mut low,
                    &mut timer,
                    &mut |u, v| {
                        bridges.push((u.min(v), u.max(v)));
                    },
                );
            }
        }

        bridges
    }
}

impl AlgorithmTrait for Bridge {
    fn name(&self) -> &'static str {
        "bridge"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl GraphAlgorithmTrait for Bridge {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge() {
        let adj = vec![
            vec![1, 2],
            vec![0, 2],
            vec![0, 1, 3],
            vec![2],
        ];
        let bridges = Bridge::find(4, &adj);
        assert_eq!(bridges, vec![(2, 3)]);
    }
}
