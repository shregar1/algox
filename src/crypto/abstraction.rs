use crate::abstraction::AlgorithmTrait;

/// Trait specific to cryptographic algorithms.
pub trait CryptoAlgorithmTrait: AlgorithmTrait {
    /// Name or identifier of the cryptographic scheme.
    fn scheme(&self) -> &'static str {
        self.name()
    }
}
