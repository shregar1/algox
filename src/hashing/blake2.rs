use crate::abstraction::AlgorithmTrait;
use super::abstraction::HashingAlgorithmTrait;
use blake2::{Blake2b512, Blake2s256, Digest};

pub struct Blake2b;

impl Blake2b {
    pub fn digest(bytes: &[u8]) -> [u8; 64] {
        let mut hasher = Blake2b512::new();
        hasher.update(bytes);
        hasher.finalize().into()
    }
}

impl AlgorithmTrait for Blake2b {
    fn name(&self) -> &'static str {
        "blake2b"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for Blake2b {
    type Output = [u8; 64];

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        Self::digest(bytes)
    }
}

pub struct Blake2s;

impl Blake2s {
    pub fn digest(bytes: &[u8]) -> [u8; 32] {
        let mut hasher = Blake2s256::new();
        hasher.update(bytes);
        hasher.finalize().into()
    }
}

impl AlgorithmTrait for Blake2s {
    fn name(&self) -> &'static str {
        "blake2s"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for Blake2s {
    type Output = [u8; 32];

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        Self::digest(bytes)
    }
}

pub fn blake2b(bytes: &[u8]) -> [u8; 64] {
    Blake2b::digest(bytes)
}

pub fn blake2s(bytes: &[u8]) -> [u8; 32] {
    Blake2s::digest(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake2() {
        assert_eq!(blake2b(b"hello").len(), 64);
        assert_eq!(blake2s(b"hello").len(), 32);
    }
}
