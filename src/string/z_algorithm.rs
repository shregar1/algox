use crate::abstraction::AlgorithmTrait;
use super::abstraction::StringAlgorithmTrait;

/// Z-Algorithm for linear time string matching in O(n + m).
pub struct ZAlgorithm;

impl ZAlgorithm {
    /// Computes the Z-array for string `s`. `z[i]` is the length of the longest
    /// common prefix between `s` and `s[i..]`.
    pub fn compute_z(s: &str) -> Vec<usize> {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut z = vec![0; n];
        if n == 0 {
            return z;
        }

        let (mut l, mut r) = (0, 0);
        for i in 1..n {
            if i <= r {
                z[i] = z[i - l].min(r - i + 1);
            }
            while i + z[i] < n && chars[z[i]] == chars[i + z[i]] {
                z[i] += 1;
            }
            if i + z[i] - 1 > r {
                l = i;
                r = i + z[i] - 1;
            }
        }
        z
    }

    /// Search for all occurrences of `pattern` in `text` returning starting indices.
    pub fn search(pattern: &str, text: &str) -> Vec<usize> {
        if pattern.is_empty() || text.is_empty() {
            return Vec::new();
        }
        let concat = format!("{}${}", pattern, text);
        let z = Self::compute_z(&concat);
        let p_len = pattern.chars().count();
        let mut matches = Vec::new();

        for i in (p_len + 1)..z.len() {
            if z[i] == p_len {
                matches.push(i - p_len - 1);
            }
        }
        matches
    }
}

impl AlgorithmTrait for ZAlgorithm {
    fn name(&self) -> &'static str {
        "z_algorithm"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl StringAlgorithmTrait for ZAlgorithm {
    fn compute(&self, text: &str, pattern: &str) -> usize {
        Self::search(pattern, text).len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z_algorithm_search() {
        let matches = ZAlgorithm::search("aba", "ababa");
        assert_eq!(matches, vec![0, 2]);
    }
}
