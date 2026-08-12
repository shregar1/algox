use super::super::abstraction::CryptoAlgorithmTrait;

/// Trait specific to ECDSA algorithms.
pub trait EcdsaAlgorithmTrait: CryptoAlgorithmTrait {
    /// Curve name (e.g. "P-256", "P-384").
    fn curve_name(&self) -> &'static str;
}
