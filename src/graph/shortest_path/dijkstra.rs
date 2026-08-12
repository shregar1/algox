use crate::abstraction::AlgorithmTrait;
use super::super::abstraction::GraphAlgorithmTrait;
use super::abstraction::ShortestPathAlgorithmTrait;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Dijkstra;

impl Dijkstra {
    pub fn shortest_path(
        nodes: usize,
        adj: &[Vec<(usize, usize)>],
        start: usize,
        target: usize,
    ) -> Option<usize> {
        let mut dist: Vec<_> = (0..nodes).map(|_| usize::MAX).collect();
        let mut heap = BinaryHeap::new();

        dist[start] = 0;
        heap.push(State {
            cost: 0,
            position: start,
        });

        while let Some(State { cost, position }) = heap.pop() {
            if position == target {
                return Some(cost);
            }
            if cost > dist[position] {
                continue;
            }

            for &(neighbor, weight) in &adj[position] {
                let next = State {
                    cost: cost + weight,
                    position: neighbor,
                };
                if next.cost < dist[neighbor] {
                    heap.push(next);
                    dist[neighbor] = next.cost;
                }
            }
        }

        None
    }
}

impl AlgorithmTrait for Dijkstra {
    fn name(&self) -> &'static str {
        "dijkstra"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl GraphAlgorithmTrait for Dijkstra {}

impl ShortestPathAlgorithmTrait for Dijkstra {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let adj = vec![
            vec![(1, 4), (2, 2)],
            vec![(2, 5), (3, 10)],
            vec![(4, 3)],
            vec![(5, 11)],
            vec![(3, 4)],
            vec![],
        ];
        assert_eq!(Dijkstra::shortest_path(6, &adj, 0, 3), Some(9));
    }
}
