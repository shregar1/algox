use crate::abstraction::AlgorithmTrait;
use super::abstraction::CryptoAlgorithmTrait;
use rsa::{Oaep, Pss, RsaPrivateKey, RsaPublicKey};
use sha2::Sha256;
use rand::thread_rng;

pub struct Rsa2048;
pub struct Rsa3072;
pub struct Rsa4096;
pub struct RsaOaep;
pub struct RsaPss;

impl Rsa2048 {
    pub fn generate_keypair() -> Result<(RsaPrivateKey, RsaPublicKey), String> {
        let mut rng = thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, 2048).map_err(|e| e.to_string())?;
        let public_key = RsaPublicKey::from(&private_key);
        Ok((private_key, public_key))
    }
}

impl AlgorithmTrait for Rsa2048 {
    fn name(&self) -> &'static str {
        "rsa-2048"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl CryptoAlgorithmTrait for Rsa2048 {}

impl Rsa3072 {
    pub fn generate_keypair() -> Result<(RsaPrivateKey, RsaPublicKey), String> {
        let mut rng = thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, 3072).map_err(|e| e.to_string())?;
        let public_key = RsaPublicKey::from(&private_key);
        Ok((private_key, public_key))
    }
}

impl AlgorithmTrait for Rsa3072 {
    fn name(&self) -> &'static str {
        "rsa-3072"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl CryptoAlgorithmTrait for Rsa3072 {}

impl Rsa4096 {
    pub fn generate_keypair() -> Result<(RsaPrivateKey, RsaPublicKey), String> {
        let mut rng = thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, 4096).map_err(|e| e.to_string())?;
        let public_key = RsaPublicKey::from(&private_key);
        Ok((private_key, public_key))
    }
}

impl AlgorithmTrait for Rsa4096 {
    fn name(&self) -> &'static str {
        "rsa-4096"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl CryptoAlgorithmTrait for Rsa4096 {}

impl RsaOaep {
    pub fn encrypt(public_key: &RsaPublicKey, plaintext: &[u8]) -> Result<Vec<u8>, String> {
        let mut rng = thread_rng();
        let padding = Oaep::new::<Sha256>();
        public_key.encrypt(&mut rng, padding, plaintext).map_err(|e| e.to_string())
    }

    pub fn decrypt(private_key: &RsaPrivateKey, ciphertext: &[u8]) -> Result<Vec<u8>, String> {
        let padding = Oaep::new::<Sha256>();
        private_key.decrypt(padding, ciphertext).map_err(|e| e.to_string())
    }
}

impl AlgorithmTrait for RsaOaep {
    fn name(&self) -> &'static str {
        "rsa-oaep"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl CryptoAlgorithmTrait for RsaOaep {}

impl RsaPss {
    pub fn sign(private_key: &RsaPrivateKey, hashed_message: &[u8]) -> Result<Vec<u8>, String> {
        let padding = Pss::new::<Sha256>();
        private_key.sign(padding, hashed_message).map_err(|e| e.to_string())
    }

    pub fn verify(public_key: &RsaPublicKey, hashed_message: &[u8], signature: &[u8]) -> Result<(), String> {
        let padding = Pss::new::<Sha256>();
        public_key.verify(padding, hashed_message, signature).map_err(|e| e.to_string())
    }
}

impl AlgorithmTrait for RsaPss {
    fn name(&self) -> &'static str {
        "rsa-pss"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl CryptoAlgorithmTrait for RsaPss {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsa_oaep() {
        let (priv_key, pub_key) = Rsa2048::generate_keypair().unwrap();
        let msg = b"rsa test message";
        let enc = RsaOaep::encrypt(&pub_key, msg).unwrap();
        let dec = RsaOaep::decrypt(&priv_key, &enc).unwrap();
        assert_eq!(dec, msg);
    }
}
