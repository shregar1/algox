use crate::abstraction::AlgorithmTrait;
use super::abstraction::StringAlgorithmTrait;

/// Levenshtein distance calculation backed by the production-grade `strsim` crate.
pub struct Levenshtein;

impl Levenshtein {
    pub fn distance(a: &str, b: &str) -> usize {
        strsim::levenshtein(a, b)
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
