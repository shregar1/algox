use crate::abstraction::AlgorithmTrait;
use super::abstraction::{GeometryAlgorithmTrait, Point};

/// Line segment intersection and point-in-polygon tests.
pub struct LineGeometry;

impl LineGeometry {
    /// Returns the cross product of vectors (p→q) and (p→r).
    fn cross(p: Point, q: Point, r: Point) -> f64 {
        (q.x - p.x) * (r.y - p.y) - (q.y - p.y) * (r.x - p.x)
    }

    fn on_segment(p: Point, q: Point, r: Point) -> bool {
        q.x <= p.x.max(r.x) && q.x >= p.x.min(r.x)
            && q.y <= p.y.max(r.y) && q.y >= p.y.min(r.y)
    }

    /// Returns `true` if segment (p1,q1) intersects segment (p2,q2).
    pub fn segments_intersect(p1: Point, q1: Point, p2: Point, q2: Point) -> bool {
        let d1 = Self::cross(p2, q2, p1);
        let d2 = Self::cross(p2, q2, q1);
        let d3 = Self::cross(p1, q1, p2);
        let d4 = Self::cross(p1, q1, q2);

        if ((d1 > 0.0 && d2 < 0.0) || (d1 < 0.0 && d2 > 0.0))
            && ((d3 > 0.0 && d4 < 0.0) || (d3 < 0.0 && d4 > 0.0))
        {
            return true;
        }
        if d1 == 0.0 && Self::on_segment(p2, p1, q2) { return true; }
        if d2 == 0.0 && Self::on_segment(p2, q1, q2) { return true; }
        if d3 == 0.0 && Self::on_segment(p1, p2, q1) { return true; }
        if d4 == 0.0 && Self::on_segment(p1, q2, q1) { return true; }
        false
    }

    /// Returns `true` if `point` is inside (or on the boundary of) the polygon
    /// defined by `vertices` (in order, open polygon — last vertex not repeated).
    /// Uses ray casting algorithm.
    pub fn point_in_polygon(point: Point, polygon: &[Point]) -> bool {
        let n = polygon.len();
        if n < 3 { return false; }
        let mut inside = false;
        let (px, py) = (point.x, point.y);
        let mut j = n - 1;
        for i in 0..n {
            let (xi, yi) = (polygon[i].x, polygon[i].y);
            let (xj, yj) = (polygon[j].x, polygon[j].y);
            if ((yi > py) != (yj > py))
                && (px < (xj - xi) * (py - yi) / (yj - yi) + xi)
            {
                inside = !inside;
            }
            j = i;
        }
        inside
    }
}

impl AlgorithmTrait for LineGeometry {
    fn name(&self) -> &'static str {
        "line_geometry"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl GeometryAlgorithmTrait for LineGeometry {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segments_intersect() {
        let p1 = Point::new(1.0, 1.0);
        let q1 = Point::new(10.0, 1.0);
        let p2 = Point::new(1.0, 2.0);
        let q2 = Point::new(10.0, 2.0);
        assert!(!LineGeometry::segments_intersect(p1, q1, p2, q2));

        let p3 = Point::new(5.0, 0.0);
        let q3 = Point::new(5.0, 5.0);
        assert!(LineGeometry::segments_intersect(p1, q1, p3, q3));
    }

    #[test]
    fn test_point_in_polygon() {
        let square = vec![
            Point::new(0.0, 0.0),
            Point::new(4.0, 0.0),
            Point::new(4.0, 4.0),
            Point::new(0.0, 4.0),
        ];
        assert!(LineGeometry::point_in_polygon(Point::new(2.0, 2.0), &square));
        assert!(!LineGeometry::point_in_polygon(Point::new(5.0, 5.0), &square));
    }
}
