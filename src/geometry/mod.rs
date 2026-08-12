pub mod abstraction;
pub mod closest_pair;
pub mod convex_hull;
pub mod line_geometry;

pub use abstraction::{GeometryAlgorithmTrait, Point};
pub use closest_pair::ClosestPair;
pub use convex_hull::ConvexHull;
pub use line_geometry::LineGeometry;
