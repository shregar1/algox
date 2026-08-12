use crate::abstraction::AlgorithmTrait;
use super::abstraction::StringAlgorithmTrait;

pub struct RabinKarp;

impl RabinKarp {
    pub fn search(text: &str, pattern: &str) -> Vec<usize> {
        let mut matches = Vec::new();
        let n = text.len();
        let m = pattern.len();

        if m == 0 || n < m {
            return matches;
        }

        let base: u64 = 256;
        let prime: u64 = 101;

        let text_bytes = text.as_bytes();
        let pat_bytes = pattern.as_bytes();

        let mut h: u64 = 1;
        for _ in 0..m - 1 {
            h = (h * base) % prime;
        }

        let mut p_hash: u64 = 0;
        let mut t_hash: u64 = 0;

        for i in 0..m {
            p_hash = (base * p_hash + pat_bytes[i] as u64) % prime;
            t_hash = (base * t_hash + text_bytes[i] as u64) % prime;
        }

        for i in 0..=n - m {
            if p_hash == t_hash && &text_bytes[i..i + m] == pat_bytes {
                matches.push(i);
            }

            if i < n - m {
                t_hash = (base * (t_hash + prime - (text_bytes[i] as u64 * h) % prime)
                    + text_bytes[i + m] as u64)
                    % prime;
            }
        }

        matches
    }
}

impl AlgorithmTrait for RabinKarp {
    fn name(&self) -> &'static str {
        "rabin_karp"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl StringAlgorithmTrait for RabinKarp {
    fn compute(&self, text: &str, pattern: &str) -> usize {
        Self::search(text, pattern).len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rabin_karp() {
        let matches = RabinKarp::search("abracadabra", "abra");
        assert_eq!(matches, vec![0, 7]);
    }
}
