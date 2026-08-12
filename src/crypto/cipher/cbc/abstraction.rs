use super::super::abstraction::CryptoAlgorithmTrait;

/// Trait specific to CBC cipher family algorithms.
pub trait CbcAlgorithmTrait: CryptoAlgorithmTrait {
    /// Encrypt plaintext bytes using key and IV.
    fn encrypt(&self, key: &[u8], iv: &[u8], plaintext: &[u8]) -> Vec<u8>;

    /// Decrypt ciphertext bytes using key and IV.
    fn decrypt(&self, key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, String>;
}
