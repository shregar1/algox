use crate::abstraction::AlgorithmTrait;
use super::abstraction::CryptoAlgorithmTrait;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes128Gcm as Aes128GcmCipher, Aes256Gcm as Aes256GcmCipher, Nonce,
};

pub struct Aes128Gcm;

impl Aes128Gcm {
    pub fn encrypt(key: &[u8; 16], nonce: &[u8; 12], plaintext: &[u8]) -> Result<Vec<u8>, String> {
        let cipher = Aes128GcmCipher::new(key.into());
        cipher
            .encrypt(Nonce::from_slice(nonce), plaintext)
            .map_err(|e| e.to_string())
    }

    pub fn decrypt(key: &[u8; 16], nonce: &[u8; 12], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
        let cipher = Aes128GcmCipher::new(key.into());
        cipher
            .decrypt(Nonce::from_slice(nonce), ciphertext)
            .map_err(|e| e.to_string())
    }
}

impl AlgorithmTrait for Aes128Gcm {
    fn name(&self) -> &'static str {
        "aes-128-gcm"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl CryptoAlgorithmTrait for Aes128Gcm {}

pub struct Aes256Gcm;

impl Aes256Gcm {
    pub fn encrypt(key: &[u8; 32], nonce: &[u8; 12], plaintext: &[u8]) -> Result<Vec<u8>, String> {
        let cipher = Aes256GcmCipher::new(key.into());
        cipher
            .encrypt(Nonce::from_slice(nonce), plaintext)
            .map_err(|e| e.to_string())
    }

    pub fn decrypt(key: &[u8; 32], nonce: &[u8; 12], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
        let cipher = Aes256GcmCipher::new(key.into());
        cipher
            .decrypt(Nonce::from_slice(nonce), ciphertext)
            .map_err(|e| e.to_string())
    }
}

impl AlgorithmTrait for Aes256Gcm {
    fn name(&self) -> &'static str {
        "aes-256-gcm"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl CryptoAlgorithmTrait for Aes256Gcm {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_gcm() {
        let key128 = [1u8; 16];
        let nonce = [2u8; 12];
        let data = b"hello gcm world!";

        let enc = Aes128Gcm::encrypt(&key128, &nonce, data).unwrap();
        let dec = Aes128Gcm::decrypt(&key128, &nonce, &enc).unwrap();
        assert_eq!(dec, data);

        let key256 = [3u8; 32];
        let enc2 = Aes256Gcm::encrypt(&key256, &nonce, data).unwrap();
        let dec2 = Aes256Gcm::decrypt(&key256, &nonce, &enc2).unwrap();
        assert_eq!(dec2, data);
    }
}
