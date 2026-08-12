use crate::abstraction::AlgorithmTrait;
use super::abstraction::CompressionAlgorithmTrait;
use flate2::read::{DeflateDecoder, DeflateEncoder};
use flate2::Compression;
use std::io::Read;

pub struct Deflate;

impl Deflate {
    pub fn compress(data: &[u8]) -> Result<Vec<u8>, String> {
        let mut encoder = DeflateEncoder::new(data, Compression::fast());
        let mut buffer = Vec::new();
        encoder.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
        Ok(buffer)
    }

    pub fn decompress(data: &[u8]) -> Result<Vec<u8>, String> {
        let mut decoder = DeflateDecoder::new(data);
        let mut buffer = Vec::new();
        decoder.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
        Ok(buffer)
    }
}

impl AlgorithmTrait for Deflate {
    fn name(&self) -> &'static str {
        "deflate"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl CompressionAlgorithmTrait for Deflate {
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
    fn test_deflate() {
        let data = b"hello deflate compression world!";
        let compressed = Deflate::compress(data).unwrap();
        let decompressed = Deflate::decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }
}
