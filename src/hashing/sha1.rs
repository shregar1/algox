use crate::abstraction::AlgorithmTrait;
use super::abstraction::HashingAlgorithmTrait;
use sha1::{Digest, Sha1 as Sha1Hasher};

pub struct Sha1;

impl Sha1 {
    pub fn digest(bytes: &[u8]) -> [u8; 20] {
        let mut hasher = Sha1Hasher::new();
        hasher.update(bytes);
        hasher.finalize().into()
    }
}

impl AlgorithmTrait for Sha1 {
    fn name(&self) -> &'static str {
        "sha1"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for Sha1 {
    type Output = [u8; 20];

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        Self::digest(bytes)
    }
}

pub fn sha1(bytes: &[u8]) -> [u8; 20] {
    Sha1::digest(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha1() {
        let digest = sha1(b"hello");
        assert_eq!(digest.len(), 20);
        assert_eq!(sha1(b"hello"), sha1(b"hello"));
    }
}
