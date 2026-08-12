use crate::abstraction::AlgorithmTrait;

/// A job/interval with a start time, finish time, and optional weight/profit.
#[derive(Debug, Clone, Copy)]
pub struct Job {
    pub start: i64,
    pub finish: i64,
    pub weight: i64,
}

impl Job {
    pub fn new(start: i64, finish: i64, weight: i64) -> Self {
        Self { start, finish, weight }
    }
}

/// Trait for scheduling algorithms.
pub trait SchedulingAlgorithmTrait: AlgorithmTrait {}
