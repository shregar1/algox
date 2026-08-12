use crate::abstraction::AlgorithmTrait;
use super::abstraction::HashingAlgorithmTrait;

pub struct Blake3;

impl Blake3 {
    pub fn digest(bytes: &[u8]) -> [u8; 32] {
        blake3::hash(bytes).into()
    }
}

impl AlgorithmTrait for Blake3 {
    fn name(&self) -> &'static str {
        "blake3"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for Blake3 {
    type Output = [u8; 32];

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        Self::digest(bytes)
    }
}

pub fn blake3(bytes: &[u8]) -> [u8; 32] {
    Blake3::digest(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3() {
        let h = blake3(b"hello");
        assert_eq!(h.len(), 32);
        assert_eq!(blake3(b"hello"), blake3(b"hello"));
    }
}
