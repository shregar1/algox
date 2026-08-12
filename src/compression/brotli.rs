use crate::abstraction::AlgorithmTrait;
use super::abstraction::CompressionAlgorithmTrait;
use std::io::Read;

pub struct Brotli;

impl Brotli {
    pub fn compress(data: &[u8]) -> Result<Vec<u8>, String> {
        let mut writer = Vec::new();
        brotli::CompressorReader::new(data, 4096, 4, 22)
            .read_to_end(&mut writer)
            .map_err(|e| e.to_string())?;
        Ok(writer)
    }

    pub fn decompress(data: &[u8]) -> Result<Vec<u8>, String> {
        let mut writer = Vec::new();
        brotli::Decompressor::new(data, 4096)
            .read_to_end(&mut writer)
            .map_err(|e| e.to_string())?;
        Ok(writer)
    }
}

impl AlgorithmTrait for Brotli {
    fn name(&self) -> &'static str {
        "brotli"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl CompressionAlgorithmTrait for Brotli {
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
    fn test_brotli() {
        let data = b"hello brotli compression world!";
        let compressed = Brotli::compress(data).unwrap();
        let decompressed = Brotli::decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }
}
