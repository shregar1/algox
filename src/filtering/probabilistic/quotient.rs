use crate::abstraction::AlgorithmTrait;
use super::super::abstraction::FilterAlgorithmTrait;
use super::abstraction::ProbabilisticFilterAlgorithmTrait;

pub struct QuotientFilter {
    slots: Vec<u64>,
    q: u8,
    r: u8,
}

impl QuotientFilter {
    pub fn new(q: u8, r: u8) -> Self {
        let q = q.clamp(3, 16);
        let r = r.clamp(3, 16);
        let num_slots = 1 << q;
        Self {
            slots: vec![0; num_slots],
            q,
            r,
        }
    }

    pub fn add(&mut self, item: &[u8]) {
        let h = Self::hash(item);
        let mask_q = (1u64 << self.q) - 1;
        let mask_r = (1u64 << self.r) - 1;

        let canonical_slot = ((h >> self.r) & mask_q) as usize;
        let remainder = (h & mask_r) as u64;

        self.slots[canonical_slot] = (remainder << 3) | 0b001; // Occupied & Continuation bit payload
    }

    pub fn contains(&self, item: &[u8]) -> bool {
        let h = Self::hash(item);
        let mask_q = (1u64 << self.q) - 1;
        let mask_r = (1u64 << self.r) - 1;

        let canonical_slot = ((h >> self.r) & mask_q) as usize;
        let remainder = (h & mask_r) as u64;

        let slot_val = self.slots[canonical_slot];
        if (slot_val & 0b001) == 0 {
            return false;
        }

        (slot_val >> 3) == remainder
    }

    pub fn clear(&mut self) {
        for slot in self.slots.iter_mut() {
            *slot = 0;
        }
    }

    fn hash(item: &[u8]) -> u64 {
        let mut h = 0xcbf29ce484222325u64;
        for &b in item {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        h
    }
}

impl AlgorithmTrait for QuotientFilter {
    fn name(&self) -> &'static str {
        "quotient_filter"
    }

    fn len(&self) -> usize {
        self.slots.iter().filter(|&&s| s != 0).count()
    }

    fn is_empty(&self) -> bool {
        self.slots.iter().all(|&s| s == 0)
    }

    fn clear(&mut self) {
        self.clear();
    }
}

impl FilterAlgorithmTrait for QuotientFilter {
    fn add(&mut self, item: &[u8]) {
        self.add(item);
    }

    fn contains(&self, item: &[u8]) -> bool {
        self.contains(item)
    }
}

impl ProbabilisticFilterAlgorithmTrait for QuotientFilter {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quotient_filter() {
        let mut qf = QuotientFilter::new(8, 8);
        qf.add(b"hello");
        assert!(qf.contains(b"hello"));
        assert!(!qf.contains(b"world"));
    }
}
