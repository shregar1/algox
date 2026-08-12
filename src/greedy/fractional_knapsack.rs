use crate::abstraction::AlgorithmTrait;
use super::abstraction::GreedyAlgorithmTrait;

/// Item with value and weight for Fractional Knapsack.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Item {
    pub value: f64,
    pub weight: f64,
}

/// Fractional Knapsack solver in O(n log n).
pub struct FractionalKnapsack;

impl FractionalKnapsack {
    /// Returns max total value achievable within `capacity`.
    pub fn max_value(items: &[Item], capacity: f64) -> f64 {
        if capacity <= 0.0 || items.is_empty() {
            return 0.0;
        }

        let mut sorted = items.to_vec();
        sorted.sort_by(|a, b| {
            let ratio_a = a.value / a.weight;
            let ratio_b = b.value / b.weight;
            ratio_b.partial_cmp(&ratio_a).unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut current_capacity = capacity;
        let mut total_value = 0.0;

        for item in sorted {
            if current_capacity <= 0.0 {
                break;
            }
            if item.weight <= current_capacity {
                current_capacity -= item.weight;
                total_value += item.value;
            } else {
                total_value += item.value * (current_capacity / item.weight);
                current_capacity = 0.0;
            }
        }
        total_value
    }
}

impl AlgorithmTrait for FractionalKnapsack {
    fn name(&self) -> &'static str {
        "fractional_knapsack"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl GreedyAlgorithmTrait for FractionalKnapsack {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fractional_knapsack() {
        let items = vec![
            Item { value: 60.0, weight: 10.0 },
            Item { value: 100.0, weight: 20.0 },
            Item { value: 120.0, weight: 30.0 },
        ];
        let val = FractionalKnapsack::max_value(&items, 50.0);
        assert_eq!(val, 240.0);
    }
}
