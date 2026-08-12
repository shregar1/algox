use crate::abstraction::AlgorithmTrait;
use super::super::abstraction::FilterAlgorithmTrait;
use super::abstraction::ProbabilisticFilterAlgorithmTrait;

pub struct HyperLogLog {
    registers: Vec<u8>,
    p: u8,
    m: usize,
}

impl HyperLogLog {
    pub fn new(p: u8) -> Self {
        let p = p.clamp(4, 16);
        let m = 1 << p;
        Self {
            registers: vec![0; m],
            p,
            m,
        }
    }

    pub fn add(&mut self, item: &[u8]) {
        let hash = Self::hash(item);
        let idx = (hash & ((self.m - 1) as u64)) as usize;
        let w = hash >> self.p;
        let rho = (w.trailing_zeros() + 1).min(64) as u8;

        if rho > self.registers[idx] {
            self.registers[idx] = rho;
        }
    }

    pub fn count(&self) -> u64 {
        let alpha_m = match self.m {
            16 => 0.673,
            32 => 0.697,
            64 => 0.709,
            _ => 0.7213 / (1.0 + 1.079 / self.m as f64),
        };

        let sum: f64 = self.registers.iter().map(|&r| 2.0f64.powi(-(r as i32))).sum();
        let mut estimate = alpha_m * (self.m as f64) * (self.m as f64) / sum;

        if estimate <= 2.5 * (self.m as f64) {
            let zeros = self.registers.iter().filter(|&&r| r == 0).count();
            if zeros > 0 {
                estimate = (self.m as f64) * ((self.m as f64) / (zeros as f64)).ln();
            }
        }

        estimate.round() as u64
    }

    pub fn contains(&self, item: &[u8]) -> bool {
        let hash = Self::hash(item);
        let idx = (hash & ((self.m - 1) as u64)) as usize;
        self.registers[idx] > 0
    }

    pub fn clear(&mut self) {
        for r in self.registers.iter_mut() {
            *r = 0;
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

impl AlgorithmTrait for HyperLogLog {
    fn name(&self) -> &'static str {
        "hyper_log_log"
    }

    fn len(&self) -> usize {
        self.count() as usize
    }

    fn is_empty(&self) -> bool {
        self.registers.iter().all(|&r| r == 0)
    }

    fn clear(&mut self) {
        self.clear();
    }
}

impl FilterAlgorithmTrait for HyperLogLog {
    fn add(&mut self, item: &[u8]) {
        self.add(item);
    }

    fn contains(&self, item: &[u8]) -> bool {
        self.contains(item)
    }
}

impl ProbabilisticFilterAlgorithmTrait for HyperLogLog {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyper_log_log() {
        let mut hll = HyperLogLog::new(10);
        for i in 0..1000 {
            hll.add(i.to_string().as_bytes());
        }

        let count = hll.count();
        assert!(count > 700 && count < 1300, "count was {}", count);
    }
}
