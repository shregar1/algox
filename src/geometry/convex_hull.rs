use crate::abstraction::AlgorithmTrait;
use super::abstraction::{GeometryAlgorithmTrait, Point};

/// Graham Scan — O(n log n) convex hull.
pub struct ConvexHull;

impl ConvexHull {
    /// Returns the convex hull of `points` in counter-clockwise order.
    pub fn compute(points: &[Point]) -> Vec<Point> {
        let mut pts = points.to_vec();
        let n = pts.len();
        if n < 3 { return pts; }

        // Find lowest-then-leftmost point
        let pivot = pts.iter().cloned().reduce(|a, b| {
            if b.y < a.y || (b.y == a.y && b.x < a.x) { b } else { a }
        }).unwrap();

        // Sort by polar angle with pivot
        pts.sort_by(|a, b| {
            let cross = Self::cross(pivot, *a, *b);
            if cross != 0.0 {
                return if cross > 0.0 { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater };
            }
            let da = (a.x - pivot.x).hypot(a.y - pivot.y);
            let db = (b.x - pivot.x).hypot(b.y - pivot.y);
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut hull: Vec<Point> = Vec::new();
        for p in pts {
            while hull.len() >= 2 {
                let cross = Self::cross(hull[hull.len() - 2], hull[hull.len() - 1], p);
                if cross <= 0.0 { hull.pop(); } else { break; }
            }
            hull.push(p);
        }
        hull
    }

    /// Cross product of vectors (o→a) and (o→b).
    fn cross(o: Point, a: Point, b: Point) -> f64 {
        (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
    }
}

impl AlgorithmTrait for ConvexHull {
    fn name(&self) -> &'static str {
        "convex_hull"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl GeometryAlgorithmTrait for ConvexHull {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convex_hull() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(0.0, 1.0),
            Point::new(0.5, 0.5), // interior
        ];
        let hull = ConvexHull::compute(&points);
        assert_eq!(hull.len(), 4); // interior point excluded
    }
}
