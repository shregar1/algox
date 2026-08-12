use crate::abstraction::AlgorithmTrait;
use super::abstraction::HashingAlgorithmTrait;
use sha2::{Digest, Sha224 as Sha224Hasher, Sha256 as Sha256Hasher, Sha384 as Sha384Hasher, Sha512 as Sha512Hasher};

pub struct Sha224;

impl Sha224 {
    pub fn digest(bytes: &[u8]) -> [u8; 28] {
        let mut hasher = Sha224Hasher::new();
        hasher.update(bytes);
        hasher.finalize().into()
    }
}

impl AlgorithmTrait for Sha224 {
    fn name(&self) -> &'static str {
        "sha224"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for Sha224 {
    type Output = [u8; 28];

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        Self::digest(bytes)
    }
}

pub struct Sha256;

impl Sha256 {
    pub fn digest(bytes: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256Hasher::new();
        hasher.update(bytes);
        hasher.finalize().into()
    }
}

impl AlgorithmTrait for Sha256 {
    fn name(&self) -> &'static str {
        "sha256"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for Sha256 {
    type Output = [u8; 32];

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        Self::digest(bytes)
    }
}

pub struct Sha384;

impl Sha384 {
    pub fn digest(bytes: &[u8]) -> [u8; 48] {
        let mut hasher = Sha384Hasher::new();
        hasher.update(bytes);
        hasher.finalize().into()
    }
}

impl AlgorithmTrait for Sha384 {
    fn name(&self) -> &'static str {
        "sha384"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for Sha384 {
    type Output = [u8; 48];

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        Self::digest(bytes)
    }
}

pub struct Sha512;

impl Sha512 {
    pub fn digest(bytes: &[u8]) -> [u8; 64] {
        let mut hasher = Sha512Hasher::new();
        hasher.update(bytes);
        hasher.finalize().into()
    }
}

impl AlgorithmTrait for Sha512 {
    fn name(&self) -> &'static str {
        "sha512"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for Sha512 {
    type Output = [u8; 64];

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        Self::digest(bytes)
    }
}

pub fn sha224(bytes: &[u8]) -> [u8; 28] {
    Sha224::digest(bytes)
}

pub fn sha256(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes)
}

pub fn sha384(bytes: &[u8]) -> [u8; 48] {
    Sha384::digest(bytes)
}

pub fn sha512(bytes: &[u8]) -> [u8; 64] {
    Sha512::digest(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha2_family() {
        assert_eq!(sha224(b"hello").len(), 28);
        assert_eq!(sha256(b"hello").len(), 32);
        assert_eq!(sha384(b"hello").len(), 48);
        assert_eq!(sha512(b"hello").len(), 64);
    }
}
