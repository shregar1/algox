use crate::abstraction::AlgorithmTrait;
use super::super::abstraction::FilterAlgorithmTrait;

pub struct CuckooFilter {
    buckets: Vec<Vec<u8>>,
    bucket_capacity: usize,
    num_buckets: usize,
    size: usize,
}

impl CuckooFilter {
    pub fn new(capacity: usize) -> Self {
        let num_buckets = (capacity / 4).max(4);
        Self {
            buckets: vec![Vec::with_capacity(4); num_buckets],
            bucket_capacity: 4,
            num_buckets,
            size: 0,
        }
    }

    pub fn add(&mut self, item: &[u8]) {
        let fp = Self::fingerprint(item);
        let i1 = Self::hash(item) % self.num_buckets;
        let i2 = (i1 ^ Self::hash(&[fp])) % self.num_buckets;

        if self.buckets[i1].len() < self.bucket_capacity {
            self.buckets[i1].push(fp);
            self.size += 1;
            return;
        }

        if self.buckets[i2].len() < self.bucket_capacity {
            self.buckets[i2].push(fp);
            self.size += 1;
            return;
        }

        // Kick out an entry if full
        let mut curr_i = i1;
        let mut curr_fp = fp;
        for _ in 0..50 {
            let idx = rand_idx() % self.buckets[curr_i].len();
            std::mem::swap(&mut self.buckets[curr_i][idx], &mut curr_fp);
            curr_i = (curr_i ^ Self::hash(&[curr_fp])) % self.num_buckets;

            if self.buckets[curr_i].len() < self.bucket_capacity {
                self.buckets[curr_i].push(curr_fp);
                self.size += 1;
                return;
            }
        }
    }

    pub fn contains(&self, item: &[u8]) -> bool {
        let fp = Self::fingerprint(item);
        let i1 = Self::hash(item) % self.num_buckets;
        let i2 = (i1 ^ Self::hash(&[fp])) % self.num_buckets;

        self.buckets[i1].contains(&fp) || self.buckets[i2].contains(&fp)
    }

    pub fn remove(&mut self, item: &[u8]) -> bool {
        let fp = Self::fingerprint(item);
        let i1 = Self::hash(item) % self.num_buckets;
        let i2 = (i1 ^ Self::hash(&[fp])) % self.num_buckets;

        if let Some(pos) = self.buckets[i1].iter().position(|&x| x == fp) {
            self.buckets[i1].remove(pos);
            self.size -= 1;
            return true;
        }

        if let Some(pos) = self.buckets[i2].iter().position(|&x| x == fp) {
            self.buckets[i2].remove(pos);
            self.size -= 1;
            return true;
        }

        false
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn clear(&mut self) {
        for b in self.buckets.iter_mut() {
            b.clear();
        }
        self.size = 0;
    }

    fn fingerprint(item: &[u8]) -> u8 {
        let mut h = 0x811c9dc5u32;
        for &b in item {
            h ^= b as u32;
            h = h.wrapping_mul(0x01000193);
        }
        let fp = (h & 0xFF) as u8;
        if fp == 0 {
            1
        } else {
            fp
        }
    }

    fn hash(item: &[u8]) -> usize {
        let mut h = 0xcbf29ce484222325u64;
        for &b in item {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        h as usize
    }
}

fn rand_idx() -> usize {
    0 // Static displacement index for deterministic cuckoo kicking
}

impl AlgorithmTrait for CuckooFilter {
    fn name(&self) -> &'static str {
        "cuckoo_filter"
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

impl FilterAlgorithmTrait for CuckooFilter {
    fn add(&mut self, item: &[u8]) {
        self.add(item);
    }

    fn contains(&self, item: &[u8]) -> bool {
        self.contains(item)
    }
}

impl super::abstraction::ProbabilisticFilterAlgorithmTrait for CuckooFilter {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cuckoo_filter() {
        let mut cf = CuckooFilter::new(64);
        cf.add(b"apple");
        cf.add(b"banana");

        assert!(cf.contains(b"apple"));
        assert!(cf.contains(b"banana"));
        assert!(!cf.contains(b"cherry"));

        assert!(cf.remove(b"apple"));
        assert!(!cf.contains(b"apple"));
    }
}
