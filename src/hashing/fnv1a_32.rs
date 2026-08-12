use crate::abstraction::AlgorithmTrait;
use super::abstraction::HashingAlgorithmTrait;

pub const FNV_OFFSET_32: u32 = 0x811c9dc5;
pub const FNV_PRIME_32: u32 = 0x01000193;

pub struct Fnv1a32;

impl Fnv1a32 {
    pub fn digest(bytes: &[u8]) -> u32 {
        let mut h = FNV_OFFSET_32;
        for &b in bytes {
            h ^= b as u32;
            h = h.wrapping_mul(FNV_PRIME_32);
        }
        h
    }
}

impl AlgorithmTrait for Fnv1a32 {
    fn name(&self) -> &'static str {
        "fnv1a_32"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for Fnv1a32 {
    type Output = u32;

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        Self::digest(bytes)
    }
}

pub fn fnv1a_32(bytes: &[u8]) -> u32 {
    Fnv1a32::digest(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fnv1a_32_empty() {
        assert_eq!(fnv1a_32(b""), FNV_OFFSET_32);
    }

    #[test]
    fn test_fnv1a_32_known() {
        let h = fnv1a_32(b"a");
        assert_ne!(h, FNV_OFFSET_32);
    }

    #[test]
    fn test_fnv1a_32_deterministic() {
        assert_eq!(fnv1a_32(b"hello"), fnv1a_32(b"hello"));
    }
}
