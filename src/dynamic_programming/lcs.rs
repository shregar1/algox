use crate::abstraction::AlgorithmTrait;
use super::abstraction::DynamicProgrammingTrait;

/// Longest Common Subsequence of two sequences.
pub struct LCS;

impl LCS {
    /// Returns the length of the longest common subsequence of `a` and `b`.
    pub fn length<T: Eq>(a: &[T], b: &[T]) -> usize {
        let (m, n) = (a.len(), b.len());
        let mut dp = vec![vec![0usize; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                if a[i - 1] == b[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }
        dp[m][n]
    }

    /// Returns the actual LCS as a Vec.
    pub fn sequence<T: Eq + Clone>(a: &[T], b: &[T]) -> Vec<T> {
        let (m, n) = (a.len(), b.len());
        let mut dp = vec![vec![0usize; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                if a[i - 1] == b[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }
        // Back-track
        let mut result = Vec::new();
        let (mut i, mut j) = (m, n);
        while i > 0 && j > 0 {
            if a[i - 1] == b[j - 1] {
                result.push(a[i - 1].clone());
                i -= 1;
                j -= 1;
            } else if dp[i - 1][j] > dp[i][j - 1] {
                i -= 1;
            } else {
                j -= 1;
            }
        }
        result.reverse();
        result
    }
}

impl AlgorithmTrait for LCS {
    fn name(&self) -> &'static str {
        "lcs"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl DynamicProgrammingTrait for LCS {
    fn description(&self) -> &'static str {
        "Longest Common Subsequence: find the longest subsequence common to two sequences."
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcs_length() {
        let a = [1, 3, 4, 5, 6, 7, 8];
        let b = [1, 3, 5, 7, 9, 10];
        assert_eq!(LCS::length(&a, &b), 4); // 1,3,5,7
    }

    #[test]
    fn test_lcs_sequence() {
        let a: Vec<char> = "ABCBDAB".chars().collect();
        let b: Vec<char> = "BDCABA".chars().collect();
        let seq = LCS::sequence(&a, &b);
        assert_eq!(seq.len(), 4);
    }

    #[test]
    fn test_lcs_empty() {
        let a: Vec<i32> = vec![];
        let b = [1, 2, 3];
        assert_eq!(LCS::length(&a, &b), 0);
    }
}
