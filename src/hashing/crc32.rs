use crate::abstraction::AlgorithmTrait;
use super::abstraction::HashingAlgorithmTrait;

pub struct Crc32;

impl Crc32 {
    pub fn digest(bytes: &[u8]) -> u32 {
        let mut crc: u32 = 0xffffffff;
        for &b in bytes {
            crc ^= b as u32;
            for _ in 0..8 {
                crc = if crc & 1 != 0 {
                    (crc >> 1) ^ 0xedb88320
                } else {
                    crc >> 1
                };
            }
        }
        crc ^ 0xffffffff
    }
}

impl AlgorithmTrait for Crc32 {
    fn name(&self) -> &'static str {
        "crc32"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for Crc32 {
    type Output = u32;

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        Self::digest(bytes)
    }
}

pub fn crc32(bytes: &[u8]) -> u32 {
    Crc32::digest(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc32_empty() {
        assert_eq!(crc32(b""), 0);
    }

    #[test]
    fn test_crc32_deterministic() {
        assert_eq!(crc32(b"hello"), crc32(b"hello"));
    }

    #[test]
    fn test_crc32_distinct_inputs() {
        assert_ne!(crc32(b"hello"), crc32(b"world"));
    }

    #[test]
    fn test_crc32_known() {
        assert_eq!(crc32(b"123456789"), 0xcbf43926);
    }
}
