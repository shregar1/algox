use crate::abstraction::AlgorithmTrait;
use super::super::abstraction::CryptoAlgorithmTrait;
use super::abstraction::CbcAlgorithmTrait;
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use cbc::{Decryptor, Encryptor};

type Aes128CbcEnc = Encryptor<aes::Aes128>;
type Aes128CbcDec = Decryptor<aes::Aes128>;

pub struct Aes128Cbc;

impl Aes128Cbc {
    pub fn encrypt(key: &[u8; 16], iv: &[u8; 16], plaintext: &[u8]) -> Vec<u8> {
        Aes128CbcEnc::new(key.into(), iv.into()).encrypt_padded_vec_mut::<Pkcs7>(plaintext)
    }

    pub fn decrypt(key: &[u8; 16], iv: &[u8; 16], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
        Aes128CbcDec::new(key.into(), iv.into())
            .decrypt_padded_vec_mut::<Pkcs7>(ciphertext)
            .map_err(|e| e.to_string())
    }
}

impl AlgorithmTrait for Aes128Cbc {
    fn name(&self) -> &'static str {
        "aes-128-cbc"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl CryptoAlgorithmTrait for Aes128Cbc {}

impl CbcAlgorithmTrait for Aes128Cbc {
    fn encrypt(&self, key: &[u8], iv: &[u8], plaintext: &[u8]) -> Vec<u8> {
        let key_arr: &[u8; 16] = key.try_into().expect("AES-128 key must be 16 bytes");
        let iv_arr: &[u8; 16] = iv.try_into().expect("AES-128 IV must be 16 bytes");
        Self::encrypt(key_arr, iv_arr, plaintext)
    }

    fn decrypt(&self, key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
        let key_arr: &[u8; 16] = key.try_into().map_err(|_| "AES-128 key must be 16 bytes")?;
        let iv_arr: &[u8; 16] = iv.try_into().map_err(|_| "AES-128 IV must be 16 bytes")?;
        Self::decrypt(key_arr, iv_arr, ciphertext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes128_cbc() {
        let key = [42u8; 16];
        let iv = [7u8; 16];
        let data = b"hello aes-128-cbc world!";

        let encrypted = Aes128Cbc::encrypt(&key, &iv, data);
        let decrypted = Aes128Cbc::decrypt(&key, &iv, &encrypted).unwrap();
        assert_eq!(decrypted, data);
    }
}
