use crate::abstraction::AlgorithmTrait;
use super::abstraction::CompressionAlgorithmTrait;

pub struct Lz4;

impl Lz4 {
    pub fn compress(data: &[u8]) -> Result<Vec<u8>, String> {
        Ok(lz4_flex::compress_prepend_size(data))
    }

    pub fn decompress(data: &[u8]) -> Result<Vec<u8>, String> {
        lz4_flex::decompress_size_prepended(data).map_err(|e| e.to_string())
    }
}

impl AlgorithmTrait for Lz4 {
    fn name(&self) -> &'static str {
        "lz4"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl CompressionAlgorithmTrait for Lz4 {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        Self::compress(data)
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        Self::decompress(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lz4() {
        let data = b"hello lz4 compression world!";
        let compressed = Lz4::compress(data).unwrap();
        let decompressed = Lz4::decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }
}
