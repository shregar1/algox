use crate::abstraction::AlgorithmTrait;
use super::abstraction::SpatialAlgorithmTrait;

/// 2D Point representation for KdTree.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn distance(&self, other: &Point2D) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

enum KdNode {
    Leaf { point: Point2D },
    Internal {
        point: Point2D,
        axis: usize, // 0 for x, 1 for y
        left: Option<Box<KdNode>>,
        right: Option<Box<KdNode>>,
    },
}

/// 2D K-d Tree for spatial indexing and Nearest Neighbor search in O(log n).
pub struct KdTree {
    root: Option<Box<KdNode>>,
    size: usize,
}

impl KdTree {
    /// Builds a 2D K-d Tree from a list of points.
    pub fn build(mut points: Vec<Point2D>) -> Self {
        let size = points.len();
        let root = Self::build_recursive(&mut points, 0);
        Self { root, size }
    }

    fn build_recursive(points: &mut [Point2D], depth: usize) -> Option<Box<KdNode>> {
        if points.is_empty() {
            return None;
        }
        let axis = depth % 2;
        points.sort_by(|a, b| {
            if axis == 0 {
                a.x.partial_cmp(&b.x).unwrap()
            } else {
                a.y.partial_cmp(&b.y).unwrap()
            }
        });

        let median = points.len() / 2;
        let point = points[median];

        let left = Self::build_recursive(&mut points[..median], depth + 1);
        let right = Self::build_recursive(&mut points[median + 1..], depth + 1);

        if left.is_none() && right.is_none() {
            Some(Box::new(KdNode::Leaf { point }))
        } else {
            Some(Box::new(KdNode::Internal {
                point,
                axis,
                left,
                right,
            }))
        }
    }

    /// Finds the nearest neighbor to target point.
    pub fn nearest_neighbor(&self, target: &Point2D) -> Option<Point2D> {
        let mut best: Option<Point2D> = None;
        let mut best_dist = f64::MAX;
        if let Some(ref root) = self.root {
            Self::search_nearest(root, target, 0, &mut best, &mut best_dist);
        }
        best
    }

    fn search_nearest(
        node: &KdNode,
        target: &Point2D,
        depth: usize,
        best: &mut Option<Point2D>,
        best_dist: &mut f64,
    ) {
        match node {
            KdNode::Leaf { point } => {
                let d = target.distance(point);
                if d < *best_dist {
                    *best_dist = d;
                    *best = Some(*point);
                }
            }
            KdNode::Internal { point, axis, left, right } => {
                let d = target.distance(point);
                if d < *best_dist {
                    *best_dist = d;
                    *best = Some(*point);
                }

                let target_coord = if *axis == 0 { target.x } else { target.y };
                let node_coord = if *axis == 0 { point.x } else { point.y };

                let (first, second) = if target_coord < node_coord {
                    (left, right)
                } else {
                    (right, left)
                };

                if let Some(ref f) = first {
                    Self::search_nearest(f, target, depth + 1, best, best_dist);
                }

                if (target_coord - node_coord).abs() < *best_dist {
                    if let Some(ref s) = second {
                        Self::search_nearest(s, target, depth + 1, best, best_dist);
                    }
                }
            }
        }
    }
}

impl AlgorithmTrait for KdTree {
    fn name(&self) -> &'static str {
        "kd_tree"
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.root = None;
        self.size = 0;
    }
}

impl SpatialAlgorithmTrait for KdTree {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kd_tree_nearest_neighbor() {
        let points = vec![
            Point2D { x: 2.0, y: 3.0 },
            Point2D { x: 5.0, y: 4.0 },
            Point2D { x: 9.0, y: 6.0 },
            Point2D { x: 4.0, y: 7.0 },
            Point2D { x: 8.0, y: 1.0 },
        ];
        let tree = KdTree::build(points);
        let nearest = tree.nearest_neighbor(&Point2D { x: 9.0, y: 2.0 }).unwrap();
        assert_eq!(nearest, Point2D { x: 8.0, y: 1.0 });
    }
}
