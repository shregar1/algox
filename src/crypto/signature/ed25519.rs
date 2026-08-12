use crate::abstraction::AlgorithmTrait;
use super::abstraction::CryptoAlgorithmTrait;
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;

pub struct Ed25519;

impl Ed25519 {
    pub fn generate_keypair() -> (SigningKey, VerifyingKey) {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        (signing_key, verifying_key)
    }

    pub fn sign(signing_key: &SigningKey, msg: &[u8]) -> Signature {
        signing_key.sign(msg)
    }

    pub fn verify(verifying_key: &VerifyingKey, msg: &[u8], signature: &Signature) -> bool {
        verifying_key.verify(msg, signature).is_ok()
    }
}

impl AlgorithmTrait for Ed25519 {
    fn name(&self) -> &'static str {
        "ed25519"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl CryptoAlgorithmTrait for Ed25519 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ed25519() {
        let (sk, vk) = Ed25519::generate_keypair();
        let msg = b"ed25519 test message";
        let sig = Ed25519::sign(&sk, msg);
        assert!(Ed25519::verify(&vk, msg, &sig));
    }
}
