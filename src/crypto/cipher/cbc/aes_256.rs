use crate::abstraction::AlgorithmTrait;
use super::super::abstraction::CryptoAlgorithmTrait;
use super::abstraction::CbcAlgorithmTrait;
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use cbc::{Decryptor, Encryptor};

type Aes256CbcEnc = Encryptor<aes::Aes256>;
type Aes256CbcDec = Decryptor<aes::Aes256>;

pub struct Aes256Cbc;

impl Aes256Cbc {
    pub fn encrypt(key: &[u8; 32], iv: &[u8; 16], plaintext: &[u8]) -> Vec<u8> {
        let mut buf = vec![0u8; plaintext.len() + 16];
        buf[..plaintext.len()].copy_from_slice(plaintext);
        let ct = Aes256CbcEnc::new(key.into(), iv.into())
            .encrypt_padded_mut::<Pkcs7>(&mut buf, plaintext.len())
            .expect("padding error");
        ct.to_vec()
    }

    pub fn decrypt(key: &[u8; 32], iv: &[u8; 16], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
        let mut buf = ciphertext.to_vec();
        let pt = Aes256CbcDec::new(key.into(), iv.into())
            .decrypt_padded_mut::<Pkcs7>(&mut buf)
            .map_err(|e| e.to_string())?;
        Ok(pt.to_vec())
    }
}

impl AlgorithmTrait for Aes256Cbc {
    fn name(&self) -> &'static str {
        "aes-256-cbc"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl CryptoAlgorithmTrait for Aes256Cbc {}

impl CbcAlgorithmTrait for Aes256Cbc {
    fn encrypt(&self, key: &[u8], iv: &[u8], plaintext: &[u8]) -> Vec<u8> {
        let key_arr: &[u8; 32] = key.try_into().expect("AES-256 key must be 32 bytes");
        let iv_arr: &[u8; 16] = iv.try_into().expect("AES-256 IV must be 16 bytes");
        Self::encrypt(key_arr, iv_arr, plaintext)
    }

    fn decrypt(&self, key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
        let key_arr: &[u8; 32] = key.try_into().map_err(|_| "AES-256 key must be 32 bytes")?;
        let iv_arr: &[u8; 16] = iv.try_into().map_err(|_| "AES-256 IV must be 16 bytes")?;
        Self::decrypt(key_arr, iv_arr, ciphertext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes256_cbc() {
        let key = [99u8; 32];
        let iv = [7u8; 16];
        let data = b"hello aes-256-cbc world!";

        let encrypted = Aes256Cbc::encrypt(&key, &iv, data);
        let decrypted = Aes256Cbc::decrypt(&key, &iv, &encrypted).unwrap();
        assert_eq!(decrypted, data);
    }
}
