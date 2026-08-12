use crate::abstraction::AlgorithmTrait;
use super::abstraction::SpatialAlgorithmTrait;
use super::kd_tree::Point2D;

/// Axis-Aligned Bounding Box (AABB) for 2D Quadtree.
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl BoundingBox {
    pub fn contains(&self, p: &Point2D) -> bool {
        p.x >= self.x - self.width
            && p.x <= self.x + self.width
            && p.y >= self.y - self.height
            && p.y <= self.y + self.height
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        !(other.x - other.width > self.x + self.width
            || other.x + other.width < self.x - self.width
            || other.y - other.height > self.y + self.height
            || other.y + other.height < self.y - self.height)
    }
}

/// 2D Quadtree spatial partitioning tree for fast 2D range querying.
pub struct QuadTree {
    boundary: BoundingBox,
    capacity: usize,
    points: Vec<Point2D>,
    divided: bool,
    north_west: Option<Box<QuadTree>>,
    north_east: Option<Box<QuadTree>>,
    south_west: Option<Box<QuadTree>>,
    south_east: Option<Box<QuadTree>>,
}

impl QuadTree {
    pub fn new(boundary: BoundingBox, capacity: usize) -> Self {
        Self {
            boundary,
            capacity: capacity.max(1),
            points: Vec::new(),
            divided: false,
            north_west: None,
            north_east: None,
            south_west: None,
            south_east: None,
        }
    }

    pub fn insert(&mut self, p: Point2D) -> bool {
        if !self.boundary.contains(&p) {
            return false;
        }

        if self.points.len() < self.capacity && !self.divided {
            self.points.push(p);
            return true;
        }

        if !self.divided {
            self.subdivide();
        }

        self.north_west.as_mut().unwrap().insert(p)
            || self.north_east.as_mut().unwrap().insert(p)
            || self.south_west.as_mut().unwrap().insert(p)
            || self.south_east.as_mut().unwrap().insert(p)
    }

    fn subdivide(&mut self) {
        let x = self.boundary.x;
        let y = self.boundary.y;
        let w = self.boundary.width / 2.0;
        let h = self.boundary.height / 2.0;

        self.north_west = Some(Box::new(QuadTree::new(BoundingBox { x: x - w, y: y + h, width: w, height: h }, self.capacity)));
        self.north_east = Some(Box::new(QuadTree::new(BoundingBox { x: x + w, y: y + h, width: w, height: h }, self.capacity)));
        self.south_west = Some(Box::new(QuadTree::new(BoundingBox { x: x - w, y: y - h, width: w, height: h }, self.capacity)));
        self.south_east = Some(Box::new(QuadTree::new(BoundingBox { x: x + w, y: y - h, width: w, height: h }, self.capacity)));
        self.divided = true;

        let old_points = std::mem::take(&mut self.points);
        for p in old_points {
            self.insert(p);
        }
    }

    pub fn query_range(&self, range: &BoundingBox) -> Vec<Point2D> {
        let mut found = Vec::new();
        if !self.boundary.intersects(range) {
            return found;
        }

        for p in &self.points {
            if range.contains(p) {
                found.push(*p);
            }
        }

        if self.divided {
            found.extend(self.north_west.as_ref().unwrap().query_range(range));
            found.extend(self.north_east.as_ref().unwrap().query_range(range));
            found.extend(self.south_west.as_ref().unwrap().query_range(range));
            found.extend(self.south_east.as_ref().unwrap().query_range(range));
        }
        found
    }
}

impl AlgorithmTrait for QuadTree {
    fn name(&self) -> &'static str {
        "quadtree"
    }

    fn len(&self) -> usize {
        self.points.len()
    }

    fn clear(&mut self) {
        self.points.clear();
        self.divided = false;
        self.north_west = None;
        self.north_east = None;
        self.south_west = None;
        self.south_east = None;
    }
}

impl SpatialAlgorithmTrait for QuadTree {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadtree_query() {
        let boundary = BoundingBox { x: 0.0, y: 0.0, width: 10.0, height: 10.0 };
        let mut qt = QuadTree::new(boundary, 2);

        qt.insert(Point2D { x: 1.0, y: 1.0 });
        qt.insert(Point2D { x: 2.0, y: 2.0 });
        qt.insert(Point2D { x: 8.0, y: 8.0 });

        let search_box = BoundingBox { x: 1.5, y: 1.5, width: 2.0, height: 2.0 };
        let results = qt.query_range(&search_box);
        assert_eq!(results.len(), 2);
    }
}
