use crate::abstraction::AlgorithmTrait;
use super::abstraction::SequenceAlgorithmTrait;

/// Needleman-Wunsch Global Sequence Alignment algorithm in O(m * n).
pub struct NeedlemanWunsch;

impl NeedlemanWunsch {
    /// Computes optimal global alignment between `seq1` and `seq2`.
    /// Returns (aligned_seq1, aligned_seq2, alignment_score).
    pub fn align(
        seq1: &str,
        seq2: &str,
        match_score: i32,
        mismatch_penalty: i32,
        gap_penalty: i32,
    ) -> (String, String, i32) {
        let s1: Vec<char> = seq1.chars().collect();
        let s2: Vec<char> = seq2.chars().collect();
        let m = s1.len();
        let n = s2.len();

        let mut dp = vec![vec![0; n + 1]; m + 1];

        for i in 0..=m {
            dp[i][0] = i as i32 * gap_penalty;
        }
        for j in 0..=n {
            dp[0][j] = j as i32 * gap_penalty;
        }

        for i in 1..=m {
            for j in 1..=n {
                let sub_score = if s1[i - 1] == s2[j - 1] {
                    match_score
                } else {
                    mismatch_penalty
                };
                let score_diag = dp[i - 1][j - 1] + sub_score;
                let score_up = dp[i - 1][j] + gap_penalty;
                let score_left = dp[i][j - 1] + gap_penalty;

                dp[i][j] = score_diag.max(score_up).max(score_left);
            }
        }

        // Traceback
        let mut align1 = String::new();
        let mut align2 = String::new();
        let mut i = m;
        let mut j = n;

        while i > 0 || j > 0 {
            if i > 0 && j > 0 {
                let sub_score = if s1[i - 1] == s2[j - 1] {
                    match_score
                } else {
                    mismatch_penalty
                };
                if dp[i][j] == dp[i - 1][j - 1] + sub_score {
                    align1.push(s1[i - 1]);
                    align2.push(s2[j - 1]);
                    i -= 1;
                    j -= 1;
                    continue;
                }
            }
            if i > 0 && dp[i][j] == dp[i - 1][j] + gap_penalty {
                align1.push(s1[i - 1]);
                align2.push('-');
                i -= 1;
            } else {
                align1.push('-');
                align2.push(s2[j - 1]);
                j -= 1;
            }
        }

        let a1: String = align1.chars().rev().collect();
        let a2: String = align2.chars().rev().collect();
        let score = dp[m][n];

        (a1, a2, score)
    }
}

impl AlgorithmTrait for NeedlemanWunsch {
    fn name(&self) -> &'static str {
        "needleman_wunsch"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl SequenceAlgorithmTrait for NeedlemanWunsch {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_needleman_wunsch() {
        let (a1, a2, score) = NeedlemanWunsch::align("GATTACA", "GCATGC", 2, -1, -1);
        assert_eq!(a1.len(), a2.len());
        assert!(score >= 0);
    }
}
