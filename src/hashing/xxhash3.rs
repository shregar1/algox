use crate::abstraction::AlgorithmTrait;
use super::abstraction::HashingAlgorithmTrait;

pub struct XxHash3;

impl XxHash3 {
    pub fn digest(bytes: &[u8]) -> u64 {
        xxhash_rust::xxh3::xxh3_64(bytes)
    }

    pub fn digest_128(bytes: &[u8]) -> u128 {
        xxhash_rust::xxh3::xxh3_128(bytes)
    }
}

impl AlgorithmTrait for XxHash3 {
    fn name(&self) -> &'static str {
        "xxhash3"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for XxHash3 {
    type Output = u64;

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        Self::digest(bytes)
    }
}

pub fn xxhash3(bytes: &[u8]) -> u64 {
    XxHash3::digest(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xxhash3() {
        let h = xxhash3(b"hello");
        assert_ne!(h, 0);
        assert_eq!(xxhash3(b"hello"), xxhash3(b"hello"));
    }
}
