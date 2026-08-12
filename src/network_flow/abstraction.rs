use crate::abstraction::AlgorithmTrait;

/// Trait for network-flow algorithms.
pub trait NetworkFlowAlgorithmTrait: AlgorithmTrait {
    /// Returns the maximum flow from source `s` to sink `t`.
    fn max_flow(&mut self, s: usize, t: usize) -> i64;
}
