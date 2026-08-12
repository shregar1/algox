use crate::abstraction::AlgorithmTrait;
use super::abstraction::CompressionAlgorithmTrait;
use flate2::read::{GzDecoder, GzEncoder};
use flate2::Compression;
use std::io::Read;

pub struct Gzip;

impl Gzip {
    pub fn compress(data: &[u8]) -> Result<Vec<u8>, String> {
        let mut encoder = GzEncoder::new(data, Compression::fast());
        let mut buffer = Vec::new();
        encoder.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
        Ok(buffer)
    }

    pub fn decompress(data: &[u8]) -> Result<Vec<u8>, String> {
        let mut decoder = GzDecoder::new(data);
        let mut buffer = Vec::new();
        decoder.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
        Ok(buffer)
    }
}

impl AlgorithmTrait for Gzip {
    fn name(&self) -> &'static str {
        "gzip"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl CompressionAlgorithmTrait for Gzip {
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
    fn test_gzip() {
        let data = b"hello gzip compression world!";
        let compressed = Gzip::compress(data).unwrap();
        let decompressed = Gzip::decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }
}
