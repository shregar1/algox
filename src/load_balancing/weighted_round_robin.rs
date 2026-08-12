use crate::abstraction::AlgorithmTrait;
use super::abstraction::LoadBalancingAlgorithmTrait;

pub struct WeightedRoundRobin {
    weights: Vec<usize>,
    current_index: usize,
    current_weight: usize,
}

impl WeightedRoundRobin {
    pub fn new(weights: Vec<usize>) -> Self {
        Self {
            weights,
            current_index: 0,
            current_weight: 0,
        }
    }

    pub fn set_weights(&mut self, weights: Vec<usize>) {
        self.weights = weights;
        self.current_index = 0;
        self.current_weight = 0;
    }
}

impl AlgorithmTrait for WeightedRoundRobin {
    fn name(&self) -> &'static str {
        "weighted_round_robin"
    }

    fn len(&self) -> usize {
        self.weights.len()
    }

    fn clear(&mut self) {
        self.weights.clear();
        self.current_index = 0;
        self.current_weight = 0;
    }
}

impl<T> LoadBalancingAlgorithmTrait<T> for WeightedRoundRobin {
    fn select<'a>(&mut self, targets: &'a [T]) -> Option<&'a T> {
        if targets.is_empty() || self.weights.is_empty() {
            return None;
        }

        let max_weight = *self.weights.iter().max().unwrap_or(&1);
        let gcd_weight = 1;

        loop {
            self.current_index = (self.current_index + 1) % targets.len();
            if self.current_index == 0 {
                if self.current_weight < gcd_weight {
                    self.current_weight = max_weight;
                } else {
                    self.current_weight = self.current_weight.saturating_sub(gcd_weight);
                }
                if self.current_weight == 0 {
                    self.current_weight = max_weight;
                }
            }

            let w = self.weights.get(self.current_index).copied().unwrap_or(1);
            if w >= self.current_weight {
                return Some(&targets[self.current_index]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weighted_round_robin() {
        let mut wrr = WeightedRoundRobin::new(vec![5, 1, 1]);
        let targets = vec!["serverA", "serverB", "serverC"];
        assert!(wrr.select(&targets).is_some());
    }
}
