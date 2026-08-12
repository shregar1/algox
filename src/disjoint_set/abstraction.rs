use crate::abstraction::AlgorithmTrait;

/// Trait for Disjoint-Set / Union-Find data structures.
pub trait DisjointSetAlgorithmTrait: AlgorithmTrait {
    fn find(&mut self, i: usize) -> usize;
    fn union(&mut self, i: usize, j: usize) -> bool;
}
