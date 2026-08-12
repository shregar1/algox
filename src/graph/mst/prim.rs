use crate::abstraction::AlgorithmTrait;
use super::super::abstraction::GraphAlgorithmTrait;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Edge {
    weight: usize,
    to: usize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Prim;

impl Prim {
    pub fn mst(nodes: usize, adj: &[Vec<(usize, usize)>]) -> usize {
        if nodes == 0 {
            return 0;
        }

        let mut visited = vec![false; nodes];
        let mut heap = BinaryHeap::new();
        let mut total_weight = 0;

        visited[0] = true;
        for &(to, weight) in &adj[0] {
            heap.push(Edge { weight, to });
        }

        while let Some(Edge { weight, to }) = heap.pop() {
            if visited[to] {
                continue;
            }

            visited[to] = true;
            total_weight += weight;

            for &(next_to, next_weight) in &adj[to] {
                if !visited[next_to] {
                    heap.push(Edge {
                        weight: next_weight,
                        to: next_to,
                    });
                }
            }
        }

        total_weight
    }
}

impl AlgorithmTrait for Prim {
    fn name(&self) -> &'static str {
        "prim"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl GraphAlgorithmTrait for Prim {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prim() {
        let adj = vec![
            vec![(1, 10), (2, 6), (3, 5)],
            vec![(0, 10), (3, 15)],
            vec![(0, 6), (3, 4)],
            vec![(0, 5), (1, 15), (2, 4)],
        ];
        let weight = Prim::mst(4, &adj);
        assert_eq!(weight, 19);
    }
}
