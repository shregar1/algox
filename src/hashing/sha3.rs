use crate::abstraction::AlgorithmTrait;
use super::abstraction::HashingAlgorithmTrait;
use sha3::{Digest, Sha3_224 as Sha3_224Hasher, Sha3_256 as Sha3_256Hasher, Sha3_384 as Sha3_384Hasher, Sha3_512 as Sha3_512Hasher};

pub struct Sha3_224;

impl Sha3_224 {
    pub fn digest(bytes: &[u8]) -> [u8; 28] {
        let mut hasher = Sha3_224Hasher::new();
        hasher.update(bytes);
        hasher.finalize().into()
    }
}

impl AlgorithmTrait for Sha3_224 {
    fn name(&self) -> &'static str {
        "sha3_224"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for Sha3_224 {
    type Output = [u8; 28];

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        Self::digest(bytes)
    }
}

pub struct Sha3_256;

impl Sha3_256 {
    pub fn digest(bytes: &[u8]) -> [u8; 32] {
        let mut hasher = Sha3_256Hasher::new();
        hasher.update(bytes);
        hasher.finalize().into()
    }
}

impl AlgorithmTrait for Sha3_256 {
    fn name(&self) -> &'static str {
        "sha3_256"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for Sha3_256 {
    type Output = [u8; 32];

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        Self::digest(bytes)
    }
}

pub struct Sha3_384;

impl Sha3_384 {
    pub fn digest(bytes: &[u8]) -> [u8; 48] {
        let mut hasher = Sha3_384Hasher::new();
        hasher.update(bytes);
        hasher.finalize().into()
    }
}

impl AlgorithmTrait for Sha3_384 {
    fn name(&self) -> &'static str {
        "sha3_384"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for Sha3_384 {
    type Output = [u8; 48];

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        Self::digest(bytes)
    }
}

pub struct Sha3_512;

impl Sha3_512 {
    pub fn digest(bytes: &[u8]) -> [u8; 64] {
        let mut hasher = Sha3_512Hasher::new();
        hasher.update(bytes);
        hasher.finalize().into()
    }
}

impl AlgorithmTrait for Sha3_512 {
    fn name(&self) -> &'static str {
        "sha3_512"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for Sha3_512 {
    type Output = [u8; 64];

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        Self::digest(bytes)
    }
}

pub fn sha3_224(bytes: &[u8]) -> [u8; 28] {
    Sha3_224::digest(bytes)
}

pub fn sha3_256(bytes: &[u8]) -> [u8; 32] {
    Sha3_256::digest(bytes)
}

pub fn sha3_384(bytes: &[u8]) -> [u8; 48] {
    Sha3_384::digest(bytes)
}

pub fn sha3_512(bytes: &[u8]) -> [u8; 64] {
    Sha3_512::digest(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha3_family() {
        assert_eq!(sha3_224(b"hello").len(), 28);
        assert_eq!(sha3_256(b"hello").len(), 32);
        assert_eq!(sha3_384(b"hello").len(), 48);
        assert_eq!(sha3_512(b"hello").len(), 64);
    }
}
