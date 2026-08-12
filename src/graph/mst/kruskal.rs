use crate::abstraction::AlgorithmTrait;
use crate::disjoint_set::DisjointSetRank;
use super::super::abstraction::GraphAlgorithmTrait;

pub struct Kruskal;

impl Kruskal {
    pub fn mst(nodes: usize, edges: &mut [(usize, usize, usize)]) -> (usize, Vec<(usize, usize, usize)>) {
        edges.sort_by_key(|e| e.2);
        let mut dsu = DisjointSetRank::new(nodes);
        let mut mst_edges = Vec::new();
        let mut total_weight = 0;

        for &(u, v, w) in edges.iter() {
            if dsu.union(u, v) {
                total_weight += w;
                mst_edges.push((u, v, w));
            }
        }

        (total_weight, mst_edges)
    }
}

impl AlgorithmTrait for Kruskal {
    fn name(&self) -> &'static str {
        "kruskal"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl GraphAlgorithmTrait for Kruskal {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kruskal() {
        let mut edges = vec![
            (0, 1, 10),
            (0, 2, 6),
            (0, 3, 5),
            (1, 3, 15),
            (2, 3, 4),
        ];
        let (weight, mst) = Kruskal::mst(4, &mut edges);
        assert_eq!(weight, 19);
        assert_eq!(mst.len(), 3);
    }
}
