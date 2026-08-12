use crate::abstraction::AlgorithmTrait;
use super::abstraction::HashingAlgorithmTrait;
use md5::{Digest, Md5 as Md5Hasher};

pub struct Md5;

impl Md5 {
    pub fn digest(bytes: &[u8]) -> [u8; 16] {
        let mut hasher = Md5Hasher::new();
        hasher.update(bytes);
        hasher.finalize().into()
    }
}

impl AlgorithmTrait for Md5 {
    fn name(&self) -> &'static str {
        "md5"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for Md5 {
    type Output = [u8; 16];

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        Self::digest(bytes)
    }
}

pub fn md5(bytes: &[u8]) -> [u8; 16] {
    Md5::digest(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        let digest = md5(b"hello");
        assert_eq!(digest.len(), 16);
        assert_eq!(md5(b"hello"), md5(b"hello"));
    }
}
