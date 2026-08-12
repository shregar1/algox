use crate::abstraction::AlgorithmTrait;
use super::abstraction::CryptoAlgorithmTrait;
use super::cbc::Aes128Cbc;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

pub struct Fernet {
    signing_key: [u8; 16],
    encryption_key: [u8; 16],
}

impl Fernet {
    pub fn new(key32: &[u8; 32]) -> Self {
        let mut signing_key = [0u8; 16];
        let mut encryption_key = [0u8; 16];
        signing_key.copy_from_slice(&key32[..16]);
        encryption_key.copy_from_slice(&key32[16..]);
        Self {
            signing_key,
            encryption_key,
        }
    }

    pub fn encrypt_at_time(&self, data: &[u8], iv: &[u8; 16], timestamp: u64) -> Vec<u8> {
        let ciphertext = Aes128Cbc::encrypt(&self.encryption_key, iv, data);
        let mut payload = Vec::with_capacity(1 + 8 + 16 + ciphertext.len());
        payload.push(0x80); // Version byte
        payload.extend_from_slice(&timestamp.to_be_bytes());
        payload.extend_from_slice(iv);
        payload.extend_from_slice(&ciphertext);

        let mut mac = HmacSha256::new_from_slice(&self.signing_key).expect("valid key length");
        mac.update(&payload);
        let hmac = mac.finalize().into_bytes();

        let mut result = payload;
        result.extend_from_slice(&hmac);
        result
    }

    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let mut iv = [0u8; 16];
        rand::Rng::fill(&mut rand::thread_rng(), &mut iv);
        self.encrypt_at_time(data, &iv, timestamp)
    }

    pub fn decrypt(&self, token: &[u8], ttl: Option<u64>) -> Result<Vec<u8>, String> {
        if token.len() < 1 + 8 + 16 + 32 || token[0] != 0x80 {
            return Err("Invalid token format".into());
        }

        let payload_len = token.len() - 32;
        let payload = &token[..payload_len];
        let signature = &token[payload_len..];

        let mut mac = HmacSha256::new_from_slice(&self.signing_key).map_err(|e| e.to_string())?;
        mac.update(payload);
        mac.verify_slice(signature).map_err(|_| "Invalid HMAC signature".to_string())?;

        let timestamp = u64::from_be_bytes(payload[1..9].try_into().unwrap());
        if let Some(ttl_sec) = ttl {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            if now < timestamp || now - timestamp > ttl_sec {
                return Err("Token expired".into());
            }
        }

        let mut iv = [0u8; 16];
        iv.copy_from_slice(&payload[9..25]);
        let ciphertext = &payload[25..];

        Aes128Cbc::decrypt(&self.encryption_key, &iv, ciphertext)
    }
}

impl AlgorithmTrait for Fernet {
    fn name(&self) -> &'static str {
        "fernet"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl CryptoAlgorithmTrait for Fernet {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fernet() {
        let key = [7u8; 32];
        let f = Fernet::new(&key);
        let token = f.encrypt(b"secret message");
        let decrypted = f.decrypt(&token, None).unwrap();
        assert_eq!(decrypted, b"secret message");
    }
}
