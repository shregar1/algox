use crate::abstraction::AlgorithmTrait;

/// Trait for load balancing algorithms.
pub trait LoadBalancingAlgorithmTrait<T>: AlgorithmTrait {
    /// Select the next item/endpoint based on load balancing policy.
    fn select<'a>(&mut self, targets: &'a [T]) -> Option<&'a T>;
}
