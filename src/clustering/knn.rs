use crate::abstraction::AlgorithmTrait;
use super::abstraction::ClusteringAlgorithmTrait;
use std::collections::HashMap;

/// K-Nearest Neighbors (k-NN) classification algorithm.
pub struct KNearestNeighbors;

impl KNearestNeighbors {
    /// Classifies a `query` point based on majority label among its `k` nearest neighbors in `dataset`.
    pub fn classify<L: Clone + std::hash::Hash + Eq>(
        dataset: &[(Vec<f64>, L)],
        query: &[f64],
        k: usize,
    ) -> Option<L> {
        if dataset.is_empty() || k == 0 {
            return None;
        }

        let mut distances: Vec<(f64, &L)> = dataset
            .iter()
            .map(|(point, label)| (Self::euclidean_dist(point, query), label))
            .collect();

        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let k = k.min(distances.len());
        let mut label_counts = HashMap::new();

        for (_, label) in &distances[..k] {
            *label_counts.entry(*label).or_insert(0) += 1;
        }

        label_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(label, _)| label.clone())
    }

    fn euclidean_dist(a: &[f64], b: &[f64]) -> f64 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}

impl AlgorithmTrait for KNearestNeighbors {
    fn name(&self) -> &'static str {
        "k_nearest_neighbors"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl ClusteringAlgorithmTrait for KNearestNeighbors {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knn_classification() {
        let dataset = vec![
            (vec![1.0, 1.0], "A"),
            (vec![2.0, 2.0], "A"),
            (vec![9.0, 9.0], "B"),
            (vec![10.0, 10.0], "B"),
        ];

        let label = KNearestNeighbors::classify(&dataset, &[1.5, 1.5], 3).unwrap();
        assert_eq!(label, "A");
    }
}
