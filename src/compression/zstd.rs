use crate::abstraction::AlgorithmTrait;
use super::abstraction::CompressionAlgorithmTrait;

pub struct Zstd;

impl Zstd {
    pub fn compress(data: &[u8]) -> Result<Vec<u8>, String> {
        zstd::bulk::compress(data, 1).map_err(|e| e.to_string())
    }

    pub fn decompress(data: &[u8]) -> Result<Vec<u8>, String> {
        zstd::bulk::decompress(data, data.len() * 10 + 1024).map_err(|e| e.to_string())
    }
}

impl AlgorithmTrait for Zstd {
    fn name(&self) -> &'static str {
        "zstd"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl CompressionAlgorithmTrait for Zstd {
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
    fn test_zstd() {
        let data = b"hello zstd compression world!";
        let compressed = Zstd::compress(data).unwrap();
        let decompressed = Zstd::decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }
}
