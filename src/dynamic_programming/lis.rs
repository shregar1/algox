use crate::abstraction::AlgorithmTrait;
use super::abstraction::DynamicProgrammingTrait;

/// Longest Increasing Subsequence (patience-sort based O(n log n)).
pub struct LIS;

impl LIS {
    /// Returns the length of the LIS of `seq`.
    pub fn length<T: Ord>(seq: &[T]) -> usize {
        let mut tails: Vec<&T> = Vec::new();
        for x in seq {
            let pos = tails.partition_point(|&&ref t| t < x);
            if pos == tails.len() {
                tails.push(x);
            } else {
                tails[pos] = x;
            }
        }
        tails.len()
    }

    /// Returns one LIS (not necessarily the lexicographically smallest).
    pub fn sequence<T: Ord + Clone>(seq: &[T]) -> Vec<T> {
        if seq.is_empty() {
            return Vec::new();
        }
        let n = seq.len();
        let mut dp = vec![1usize; n];
        let mut prev = vec![usize::MAX; n];

        for i in 1..n {
            for j in 0..i {
                if seq[j] < seq[i] && dp[j] + 1 > dp[i] {
                    dp[i] = dp[j] + 1;
                    prev[i] = j;
                }
            }
        }

        let (mut idx, mut best) = (0, 0);
        for (i, &d) in dp.iter().enumerate() {
            if d > best { best = d; idx = i; }
        }

        let mut result = Vec::new();
        let mut cur = idx;
        loop {
            result.push(seq[cur].clone());
            if prev[cur] == usize::MAX { break; }
            cur = prev[cur];
        }
        result.reverse();
        result
    }
}

impl AlgorithmTrait for LIS {
    fn name(&self) -> &'static str {
        "lis"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl DynamicProgrammingTrait for LIS {
    fn description(&self) -> &'static str {
        "Longest Increasing Subsequence: find the longest strictly increasing subsequence."
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lis_length() {
        assert_eq!(LIS::length(&[10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }

    #[test]
    fn test_lis_sequence() {
        let seq = LIS::sequence(&[10, 9, 2, 5, 3, 7, 101, 18]);
        // One valid LIS: [2,5,7,101] or [2,3,7,101] — length 4
        assert_eq!(seq.len(), 4);
        for w in seq.windows(2) {
            assert!(w[0] < w[1]);
        }
    }
}
