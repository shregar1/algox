use crate::abstraction::AlgorithmTrait;
use super::abstraction::CryptoAlgorithmTrait;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305 as ChaCha20Poly1305Cipher, Nonce,
};

pub struct ChaCha20Poly1305;

impl ChaCha20Poly1305 {
    pub fn encrypt(key: &[u8; 32], nonce: &[u8; 12], plaintext: &[u8]) -> Result<Vec<u8>, String> {
        let cipher = ChaCha20Poly1305Cipher::new(key.into());
        cipher
            .encrypt(Nonce::from_slice(nonce), plaintext)
            .map_err(|e| e.to_string())
    }

    pub fn decrypt(key: &[u8; 32], nonce: &[u8; 12], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
        let cipher = ChaCha20Poly1305Cipher::new(key.into());
        cipher
            .decrypt(Nonce::from_slice(nonce), ciphertext)
            .map_err(|e| e.to_string())
    }
}

impl AlgorithmTrait for ChaCha20Poly1305 {
    fn name(&self) -> &'static str {
        "chacha20-poly1305"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl CryptoAlgorithmTrait for ChaCha20Poly1305 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chacha20_poly1305() {
        let key = [5u8; 32];
        let nonce = [9u8; 12];
        let data = b"hello chacha!";

        let enc = ChaCha20Poly1305::encrypt(&key, &nonce, data).unwrap();
        let dec = ChaCha20Poly1305::decrypt(&key, &nonce, &enc).unwrap();
        assert_eq!(dec, data);
    }
}
