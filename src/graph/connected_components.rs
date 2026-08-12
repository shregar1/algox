use crate::abstraction::AlgorithmTrait;
use super::abstraction::GraphAlgorithmTrait;
use super::traversal::DFS;

pub struct ConnectedComponents;

impl ConnectedComponents {
    pub fn find(nodes: usize, adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
        let mut visited = vec![false; nodes];
        let mut components = Vec::new();

        for i in 0..nodes {
            if !visited[i] {
                let mut comp = Vec::new();
                DFS::traverse_with_visited(i, adj, &mut visited, &mut comp);
                components.push(comp);
            }
        }

        components
    }
}

impl AlgorithmTrait for ConnectedComponents {
    fn name(&self) -> &'static str {
        "connected_components"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl GraphAlgorithmTrait for ConnectedComponents {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connected_components() {
        let adj = vec![
            vec![1],
            vec![0],
            vec![3],
            vec![2],
            vec![],
        ];
        let comps = ConnectedComponents::find(5, &adj);
        assert_eq!(comps.len(), 3);
    }
}
