use crate::abstraction::AlgorithmTrait;
use super::abstraction::HashingAlgorithmTrait;

pub const FNV_OFFSET_64: u64 = 0xcbf29ce484222325;
pub const FNV_PRIME_64: u64 = 0x100000001b3;

pub struct Fnv1a64;

impl Fnv1a64 {
    pub fn digest(bytes: &[u8]) -> u64 {
        let mut h = FNV_OFFSET_64;
        for &b in bytes {
            h ^= b as u64;
            h = h.wrapping_mul(FNV_PRIME_64);
        }
        h
    }
}

impl AlgorithmTrait for Fnv1a64 {
    fn name(&self) -> &'static str {
        "fnv1a_64"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for Fnv1a64 {
    type Output = u64;

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        Self::digest(bytes)
    }
}

pub fn fnv1a_64(bytes: &[u8]) -> u64 {
    Fnv1a64::digest(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fnv1a_64_empty() {
        assert_eq!(fnv1a_64(b""), FNV_OFFSET_64);
    }

    #[test]
    fn test_fnv1a_64_deterministic() {
        assert_eq!(fnv1a_64(b"hello"), fnv1a_64(b"hello"));
    }

    #[test]
    fn test_fnv1a_32_vs_64_distinct() {
        let h32 = super::super::fnv1a_32(b"hello");
        let h64 = fnv1a_64(b"hello");
        assert_ne!(h32 as u64, h64);
    }
}
