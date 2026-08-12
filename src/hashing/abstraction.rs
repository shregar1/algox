use crate::abstraction::AlgorithmTrait;

/// Trait specific to hashing algorithms.
pub trait HashingAlgorithmTrait: AlgorithmTrait {
    /// Output type of the hash calculation.
    type Output;

    /// Calculate hash for the given byte slice.
    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output;
}
