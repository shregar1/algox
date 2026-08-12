pub mod abstraction;
pub mod kd_tree;
pub mod quadtree;

pub use abstraction::SpatialAlgorithmTrait;
pub use kd_tree::{KdTree, Point2D};
pub use quadtree::{BoundingBox, QuadTree};
