use crate::abstraction::AlgorithmTrait;
use super::super::abstraction::FilterAlgorithmTrait;

pub struct BloomFilter {
    bits: Vec<u8>,
    bit_count: usize,
    hash_count: u32,
}

impl BloomFilter {
    pub fn new(bit_count: usize, hash_count: u32) -> Self {
        let bits = vec![0u8; bit_count.div_ceil(8).max(1)];
        Self {
            bits,
            bit_count,
            hash_count,
        }
    }

    pub fn with_capacity(expected_items: usize, false_positive_rate: f64) -> Self {
        let fpr = false_positive_rate.clamp(1e-9, 0.99);
        let ln2 = std::f64::consts::LN_2;
        let ln2_sq = ln2 * ln2;
        let m = -((expected_items.max(1) as f64) * fpr.ln()) / ln2_sq;
        let k = ((m / expected_items.max(1) as f64) * ln2).ceil().max(1.0) as u32;
        let bit_count = m.ceil().max(8.0) as usize;
        Self::new(bit_count, k)
    }

    pub fn add(&mut self, item: &[u8]) {
        for i in 0..self.hash_count {
            let h = Self::hash_with_seed(item, i as u64);
            let idx = (h as usize) % self.bit_count;
            let byte = idx / 8;
            let bit = idx % 8;
            self.bits[byte] |= 1 << bit;
        }
    }

    pub fn contains(&self, item: &[u8]) -> bool {
        for i in 0..self.hash_count {
            let h = Self::hash_with_seed(item, i as u64);
            let idx = (h as usize) % self.bit_count;
            let byte = idx / 8;
            let bit = idx % 8;
            if self.bits[byte] & (1 << bit) == 0 {
                return false;
            }
        }
        true
    }

    pub fn bit_count(&self) -> usize {
        self.bit_count
    }

    pub fn hash_count(&self) -> u32 {
        self.hash_count
    }

    pub fn is_empty(&self) -> bool {
        self.bits.iter().all(|b| *b == 0)
    }

    pub fn clear(&mut self) {
        for b in self.bits.iter_mut() {
            *b = 0;
        }
    }

    fn hash_with_seed(item: &[u8], seed: u64) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325 ^ seed.wrapping_mul(0x100000001b3);
        for &b in item {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        h
    }
}

impl AlgorithmTrait for BloomFilter {
    fn name(&self) -> &'static str {
        "bloom_filter"
    }

    fn len(&self) -> usize {
        self.bits.iter().map(|b| b.count_ones() as usize).sum()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn clear(&mut self) {
        self.clear();
    }
}

impl FilterAlgorithmTrait for BloomFilter {
    fn add(&mut self, item: &[u8]) {
        self.add(item);
    }

    fn contains(&self, item: &[u8]) -> bool {
        self.contains(item)
    }
}

impl super::abstraction::ProbabilisticFilterAlgorithmTrait for BloomFilter {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_contains() {
        let mut bf = BloomFilter::new(1024, 3);
        bf.add(b"hello");
        bf.add(b"world");
        assert!(bf.contains(b"hello"));
        assert!(bf.contains(b"world"));
    }

    #[test]
    fn test_missing_returns_false() {
        let mut bf = BloomFilter::new(1024, 3);
        bf.add(b"present");
        assert!(!bf.contains(b"absent"));
    }

    #[test]
    fn test_with_capacity() {
        let bf = BloomFilter::with_capacity(1000, 0.01);
        assert!(bf.bit_count() > 0);
        assert!(bf.hash_count() > 0);
    }

    #[test]
    fn test_clear() {
        let mut bf = BloomFilter::new(1024, 3);
        bf.add(b"item");
        bf.clear();
        assert!(bf.is_empty());
        assert!(!bf.contains(b"item"));
    }

    #[test]
    fn test_no_false_negatives() {
        let mut bf = BloomFilter::with_capacity(100, 0.001);
        let items: Vec<Vec<u8>> = (0..100).map(|i| i.to_string().into_bytes()).collect();
        for it in &items {
            bf.add(it);
        }
        for it in &items {
            assert!(bf.contains(it));
        }
    }
}
