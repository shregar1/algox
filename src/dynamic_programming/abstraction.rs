use crate::abstraction::AlgorithmTrait;

/// Trait for dynamic programming algorithms.
pub trait DynamicProgrammingTrait: AlgorithmTrait {
    /// Returns a human-readable description of the subproblem structure.
    fn description(&self) -> &'static str;
}
