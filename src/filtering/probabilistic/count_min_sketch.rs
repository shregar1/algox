use crate::abstraction::AlgorithmTrait;
use super::super::abstraction::FilterAlgorithmTrait;
use super::abstraction::ProbabilisticFilterAlgorithmTrait;

pub struct CountMinSketch {
    depth: usize,
    width: usize,
    table: Vec<Vec<u64>>,
    total_count: u64,
}

impl CountMinSketch {
    pub fn new(depth: usize, width: usize) -> Self {
        let depth = depth.max(1);
        let width = width.max(1);
        Self {
            depth,
            width,
            table: vec![vec![0; width]; depth],
            total_count: 0,
        }
    }

    pub fn add_item(&mut self, item: &[u8], count: u64) {
        for i in 0..self.depth {
            let h = Self::hash_with_seed(item, i as u64);
            let idx = (h as usize) % self.width;
            self.table[i][idx] = self.table[i][idx].saturating_add(count);
        }
        self.total_count = self.total_count.saturating_add(count);
    }

    pub fn estimate(&self, item: &[u8]) -> u64 {
        let mut min_val = u64::MAX;
        for i in 0..self.depth {
            let h = Self::hash_with_seed(item, i as u64);
            let idx = (h as usize) % self.width;
            min_val = min_val.min(self.table[i][idx]);
        }
        if min_val == u64::MAX {
            0
        } else {
            min_val
        }
    }

    pub fn total_count(&self) -> u64 {
        self.total_count
    }

    pub fn clear(&mut self) {
        for row in self.table.iter_mut() {
            for val in row.iter_mut() {
                *val = 0;
            }
        }
        self.total_count = 0;
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

impl AlgorithmTrait for CountMinSketch {
    fn name(&self) -> &'static str {
        "count_min_sketch"
    }

    fn len(&self) -> usize {
        self.total_count as usize
    }

    fn is_empty(&self) -> bool {
        self.total_count == 0
    }

    fn clear(&mut self) {
        self.clear();
    }
}

impl FilterAlgorithmTrait for CountMinSketch {
    fn add(&mut self, item: &[u8]) {
        self.add_item(item, 1);
    }

    fn contains(&self, item: &[u8]) -> bool {
        self.estimate(item) > 0
    }
}

impl ProbabilisticFilterAlgorithmTrait for CountMinSketch {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_min_sketch() {
        let mut cms = CountMinSketch::new(4, 256);
        cms.add_item(b"apple", 5);
        cms.add_item(b"banana", 10);

        assert!(cms.estimate(b"apple") >= 5);
        assert!(cms.estimate(b"banana") >= 10);
        assert_eq!(cms.estimate(b"cherry"), 0);
    }
}
