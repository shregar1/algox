use crate::abstraction::AlgorithmTrait;

pub struct KmpSearch;

impl KmpSearch {
    pub fn search(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        if needle.is_empty() {
            return Some(0);
        }
        if haystack.len() < needle.len() {
            return None;
        }

        let lps = Self::compute_lps(needle);
        let mut i = 0;
        let mut j = 0;

        while i < haystack.len() {
            if haystack[i] == needle[j] {
                i += 1;
                j += 1;
            }

            if j == needle.len() {
                return Some(i - j);
            } else if i < haystack.len() && haystack[i] != needle[j] {
                if j != 0 {
                    j = lps[j - 1];
                } else {
                    i += 1;
                }
            }
        }

        None
    }

    fn compute_lps(pattern: &[u8]) -> Vec<usize> {
        let mut lps = vec![0; pattern.len()];
        let mut len = 0;
        let mut i = 1;

        while i < pattern.len() {
            if pattern[i] == pattern[len] {
                len += 1;
                lps[i] = len;
                i += 1;
            } else if len != 0 {
                len = lps[len - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }

        lps
    }
}

impl AlgorithmTrait for KmpSearch {
    fn name(&self) -> &'static str {
        "kmp_search"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kmp_search() {
        let mut kmp = KmpSearch;
        assert_eq!(kmp.name(), "kmp_search");
        assert_eq!(kmp.len(), 0);
        kmp.clear();

        let text = b"ababcabcabababd";
        let pattern = b"ababd";
        assert_eq!(KmpSearch::search(text, pattern), Some(10));
        assert_eq!(KmpSearch::search(text, b"xyz"), None);

        // Edge cases
        assert_eq!(KmpSearch::search(text, b""), Some(0));
        assert_eq!(KmpSearch::search(b"hi", b"hello world"), None);
    }
}
