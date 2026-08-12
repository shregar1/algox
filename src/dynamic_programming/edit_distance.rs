use crate::abstraction::AlgorithmTrait;
use super::abstraction::DynamicProgrammingTrait;

/// Edit Distance (Levenshtein distance) between two strings.
pub struct EditDistance;

impl EditDistance {
    /// Returns the minimum number of single-character edits (insert, delete, replace)
    /// to transform `a` into `b`.
    pub fn distance(a: &str, b: &str) -> usize {
        let a: Vec<char> = a.chars().collect();
        let b: Vec<char> = b.chars().collect();
        let (m, n) = (a.len(), b.len());
        let mut dp = vec![vec![0usize; n + 1]; m + 1];
        for i in 0..=m { dp[i][0] = i; }
        for j in 0..=n { dp[0][j] = j; }
        for i in 1..=m {
            for j in 1..=n {
                if a[i - 1] == b[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]);
                }
            }
        }
        dp[m][n]
    }
}

impl AlgorithmTrait for EditDistance {
    fn name(&self) -> &'static str {
        "edit_distance"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl DynamicProgrammingTrait for EditDistance {
    fn description(&self) -> &'static str {
        "Edit Distance: minimum insert/delete/replace operations to transform one string into another."
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_distance() {
        assert_eq!(EditDistance::distance("kitten", "sitting"), 3);
        assert_eq!(EditDistance::distance("", "abc"), 3);
        assert_eq!(EditDistance::distance("abc", "abc"), 0);
        assert_eq!(EditDistance::distance("horse", "ros"), 3);
    }
}
