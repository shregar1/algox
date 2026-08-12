use crate::abstraction::AlgorithmTrait;
use super::abstraction::ClusteringAlgorithmTrait;

/// K-Means Clustering algorithm.
pub struct KMeans;

impl KMeans {
    /// Partitions `data` into `k` clusters. Returns cluster assignments for each point and final centroid coordinates.
    pub fn cluster(data: &[Vec<f64>], k: usize, max_iterations: usize) -> (Vec<usize>, Vec<Vec<f64>>) {
        if data.is_empty() || k == 0 {
            return (Vec::new(), Vec::new());
        }
        let k = k.min(data.len());
        let dim = data[0].len();

        // Initialize centroids from the first k points
        let mut centroids: Vec<Vec<f64>> = data[..k].to_vec();
        let mut assignments = vec![0; data.len()];

        for _ in 0..max_iterations {
            let mut changed = false;

            // Assignment step
            for (idx, point) in data.iter().enumerate() {
                let mut min_dist = f64::MAX;
                let mut best_c = 0;

                for (c_idx, centroid) in centroids.iter().enumerate() {
                    let d = Self::euclidean_dist(point, centroid);
                    if d < min_dist {
                        min_dist = d;
                        best_c = c_idx;
                    }
                }

                if assignments[idx] != best_c {
                    assignments[idx] = best_c;
                    changed = true;
                }
            }

            if !changed {
                break;
            }

            // Update step
            let mut sums = vec![vec![0.0; dim]; k];
            let mut counts = vec![0; k];

            for (idx, point) in data.iter().enumerate() {
                let c = assignments[idx];
                counts[c] += 1;
                for d in 0..dim {
                    sums[c][d] += point[d];
                }
            }

            for c in 0..k {
                if counts[c] > 0 {
                    for d in 0..dim {
                        centroids[c][d] = sums[c][d] / (counts[c] as f64);
                    }
                }
            }
        }

        (assignments, centroids)
    }

    fn euclidean_dist(a: &[f64], b: &[f64]) -> f64 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}

impl AlgorithmTrait for KMeans {
    fn name(&self) -> &'static str {
        "kmeans"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl ClusteringAlgorithmTrait for KMeans {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kmeans() {
        let data = vec![
            vec![1.0, 1.0],
            vec![1.5, 1.5],
            vec![10.0, 10.0],
            vec![10.5, 10.5],
        ];
        let (assignments, centroids) = KMeans::cluster(&data, 2, 100);
        assert_eq!(centroids.len(), 2);
        assert_eq!(assignments[0], assignments[1]);
        assert_eq!(assignments[2], assignments[3]);
        assert_ne!(assignments[0], assignments[2]);
    }
}
