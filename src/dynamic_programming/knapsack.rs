use crate::abstraction::AlgorithmTrait;
use super::abstraction::DynamicProgrammingTrait;

/// 0/1 Knapsack — classic DP.
/// Given item weights and values and a capacity W, find max value achievable.
pub struct Knapsack01;

impl Knapsack01 {
    /// Returns the maximum value achievable without exceeding `capacity`.
    /// `weights` and `values` must have the same length.
    pub fn solve(weights: &[usize], values: &[usize], capacity: usize) -> usize {
        let n = weights.len();
        // dp[i][w] = max value using first i items with weight budget w
        let mut dp = vec![vec![0usize; capacity + 1]; n + 1];
        for i in 1..=n {
            let wi = weights[i - 1];
            let vi = values[i - 1];
            for w in 0..=capacity {
                dp[i][w] = dp[i - 1][w];
                if wi <= w {
                    dp[i][w] = dp[i][w].max(dp[i - 1][w - wi] + vi);
                }
            }
        }
        dp[n][capacity]
    }

    /// Returns the selected item indices (0-based) that form the optimal solution.
    pub fn items(weights: &[usize], values: &[usize], capacity: usize) -> Vec<usize> {
        let n = weights.len();
        let mut dp = vec![vec![0usize; capacity + 1]; n + 1];
        for i in 1..=n {
            let wi = weights[i - 1];
            let vi = values[i - 1];
            for w in 0..=capacity {
                dp[i][w] = dp[i - 1][w];
                if wi <= w {
                    dp[i][w] = dp[i][w].max(dp[i - 1][w - wi] + vi);
                }
            }
        }
        // Back-track
        let mut selected = Vec::new();
        let mut w = capacity;
        for i in (1..=n).rev() {
            if dp[i][w] != dp[i - 1][w] {
                selected.push(i - 1);
                w -= weights[i - 1];
            }
        }
        selected.reverse();
        selected
    }
}

impl AlgorithmTrait for Knapsack01 {
    fn name(&self) -> &'static str {
        "knapsack_01"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl DynamicProgrammingTrait for Knapsack01 {
    fn description(&self) -> &'static str {
        "0/1 Knapsack: maximise value subject to weight capacity; each item used at most once."
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knapsack01_value() {
        let weights = [2, 3, 4, 5];
        let values = [3, 4, 5, 6];
        assert_eq!(Knapsack01::solve(&weights, &values, 5), 7); // items 0+1
    }

    #[test]
    fn test_knapsack01_items() {
        let weights = [2, 3, 4, 5];
        let values = [3, 4, 5, 6];
        let items = Knapsack01::items(&weights, &values, 5);
        assert_eq!(items, vec![0, 1]);
    }

    #[test]
    fn test_knapsack01_zero_capacity() {
        let weights = [1, 2];
        let values = [10, 20];
        assert_eq!(Knapsack01::solve(&weights, &values, 0), 0);
    }
}
