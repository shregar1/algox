use crate::abstraction::AlgorithmTrait;

/// Trait for compression algorithms.
pub trait CompressionAlgorithmTrait: AlgorithmTrait {
    /// Compress input bytes.
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, String>;

    /// Decompress compressed bytes.
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, String>;
}
