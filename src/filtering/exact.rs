use crate::abstraction::AlgorithmTrait;
use super::abstraction::FilterAlgorithmTrait;
use std::collections::HashSet;

pub struct ExactFilter {
    set: HashSet<Vec<u8>>,
}

impl ExactFilter {
    pub fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            set: HashSet::with_capacity(capacity),
        }
    }

    pub fn add(&mut self, item: &[u8]) {
        self.set.insert(item.to_vec());
    }

    pub fn contains(&self, item: &[u8]) -> bool {
        self.set.contains(item)
    }

    pub fn remove(&mut self, item: &[u8]) -> bool {
        self.set.remove(item)
    }

    pub fn len(&self) -> usize {
        self.set.len()
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn clear(&mut self) {
        self.set.clear();
    }
}

impl Default for ExactFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl AlgorithmTrait for ExactFilter {
    fn name(&self) -> &'static str {
        "exact_filter"
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn clear(&mut self) {
        self.clear();
    }
}

impl FilterAlgorithmTrait for ExactFilter {
    fn add(&mut self, item: &[u8]) {
        self.add(item);
    }

    fn contains(&self, item: &[u8]) -> bool {
        self.contains(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_filter() {
        let mut ef = ExactFilter::new();
        ef.add(b"hello");
        assert!(ef.contains(b"hello"));
        assert!(!ef.contains(b"world"));

        ef.remove(b"hello");
        assert!(!ef.contains(b"hello"));
    }
}
