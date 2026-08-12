use crate::abstraction::AlgorithmTrait;
use super::abstraction::StringAlgorithmTrait;
use aho_corasick::AhoCorasick as AcInner;

/// Multi-pattern search automaton backed by the SIMD-accelerated `aho-corasick` crate.
pub struct AhoCorasick {
    inner: AcInner,
    num_patterns: usize,
}

impl AhoCorasick {
    pub fn new(patterns: &[&[u8]]) -> Self {
        let inner = AcInner::new(patterns).unwrap();
        Self {
            inner,
            num_patterns: patterns.len(),
        }
    }

    pub fn find_all(&self, text: &[u8]) -> Vec<(usize, usize)> {
        self.inner
            .find_iter(text)
            .map(|mat| (mat.end() - 1, mat.pattern().as_usize()))
            .collect()
    }
}

impl AlgorithmTrait for AhoCorasick {
    fn name(&self) -> &'static str {
        "aho_corasick"
    }

    fn len(&self) -> usize {
        self.num_patterns
    }

    fn clear(&mut self) {}
}

impl StringAlgorithmTrait for AhoCorasick {
    fn compute(&self, text: &str, _pattern: &str) -> usize {
        self.find_all(text.as_bytes()).len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aho_corasick() {
        let patterns = vec![b"he".as_slice(), b"she".as_slice(), b"his".as_slice(), b"hers".as_slice()];
        let ac = AhoCorasick::new(&patterns);
        let matches = ac.find_all(b"ushers");
        assert!(!matches.is_empty());
    }
}
