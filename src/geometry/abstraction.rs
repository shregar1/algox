use crate::abstraction::AlgorithmTrait;

/// A 2-D point with f64 coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Trait for computational geometry algorithms.
pub trait GeometryAlgorithmTrait: AlgorithmTrait {}
