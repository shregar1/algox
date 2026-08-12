use crate::abstraction::AlgorithmTrait;

/// Trait for encoding/decoding algorithms.
pub trait EncodingAlgorithmTrait: AlgorithmTrait {
    /// Encode input bytes into a string.
    fn encode(&self, data: &[u8]) -> String;

    /// Decode encoded string back into raw bytes.
    fn decode(&self, encoded: &str) -> Result<Vec<u8>, String>;
}
