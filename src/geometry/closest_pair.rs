use crate::abstraction::AlgorithmTrait;
use super::abstraction::{GeometryAlgorithmTrait, Point};

/// Closest Pair of Points — divide-and-conquer O(n log n).
pub struct ClosestPair;

impl ClosestPair {
    /// Returns the minimum distance between any two points in `points`.
    pub fn min_distance(points: &[Point]) -> f64 {
        if points.len() < 2 { return f64::INFINITY; }
        let mut sorted = points.to_vec();
        sorted.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        Self::closest_rec(&sorted)
    }

    fn dist(a: Point, b: Point) -> f64 {
        (a.x - b.x).hypot(a.y - b.y)
    }

    fn brute(pts: &[Point]) -> f64 {
        let mut min = f64::INFINITY;
        for i in 0..pts.len() {
            for j in i + 1..pts.len() {
                let d = Self::dist(pts[i], pts[j]);
                if d < min { min = d; }
            }
        }
        min
    }

    fn closest_rec(pts: &[Point]) -> f64 {
        let n = pts.len();
        if n <= 3 { return Self::brute(pts); }
        let mid = n / 2;
        let mid_x = pts[mid].x;
        let d = Self::closest_rec(&pts[..mid]).min(Self::closest_rec(&pts[mid..]));

        // Strip
        let mut strip: Vec<Point> = pts.iter()
            .filter(|p| (p.x - mid_x).abs() < d)
            .cloned()
            .collect();
        strip.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

        let mut min_strip = d;
        for i in 0..strip.len() {
            let mut j = i + 1;
            while j < strip.len() && (strip[j].y - strip[i].y) < min_strip {
                let d2 = Self::dist(strip[i], strip[j]);
                if d2 < min_strip { min_strip = d2; }
                j += 1;
            }
        }
        min_strip
    }
}

impl AlgorithmTrait for ClosestPair {
    fn name(&self) -> &'static str {
        "closest_pair"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl GeometryAlgorithmTrait for ClosestPair {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closest_pair() {
        let pts = vec![
            Point::new(0.0, 0.0),
            Point::new(3.0, 4.0),
            Point::new(1.0, 1.0),
            Point::new(10.0, 10.0),
        ];
        let d = ClosestPair::min_distance(&pts);
        // (0,0)→(1,1) = √2 ≈ 1.414
        assert!((d - 2f64.sqrt()).abs() < 1e-9);
    }
}
