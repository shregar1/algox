use crate::abstraction::AlgorithmTrait;
use super::abstraction::StringAlgorithmTrait;

pub struct Levenshtein;

impl Levenshtein {
    pub fn distance(a: &str, b: &str) -> usize {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();

        let len_a = a_chars.len();
        let len_b = b_chars.len();

        if len_a == 0 {
            return len_b;
        }
        if len_b == 0 {
            return len_a;
        }

        let mut dp = vec![vec![0usize; len_b + 1]; len_a + 1];

        for i in 0..=len_a {
            dp[i][0] = i;
        }
        for j in 0..=len_b {
            dp[0][j] = j;
        }

        for i in 1..=len_a {
            for j in 1..=len_b {
                let cost = if a_chars[i - 1] == b_chars[j - 1] {
                    0
                } else {
                    1
                };
                dp[i][j] = (dp[i - 1][j] + 1)
                    .min(dp[i][j - 1] + 1)
                    .min(dp[i - 1][j - 1] + cost);
            }
        }

        dp[len_a][len_b]
    }
}

impl AlgorithmTrait for Levenshtein {
    fn name(&self) -> &'static str {
        "levenshtein"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl StringAlgorithmTrait for Levenshtein {
    fn compute(&self, a: &str, b: &str) -> usize {
        Self::distance(a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein() {
        assert_eq!(Levenshtein::distance("kitten", "sitting"), 3);
        assert_eq!(Levenshtein::distance("flaw", "lawn"), 2);
    }
}
