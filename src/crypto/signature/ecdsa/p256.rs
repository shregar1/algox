use crate::abstraction::AlgorithmTrait;
use super::super::abstraction::CryptoAlgorithmTrait;
use super::abstraction::EcdsaAlgorithmTrait;
use p256::ecdsa::{SigningKey as P256SigningKey, VerifyingKey as P256VerifyingKey, Signature as P256Signature, signature::Signer, signature::Verifier};
use rand::thread_rng;

pub struct EcdsaP256;

impl EcdsaP256 {
    pub fn generate_keypair() -> (P256SigningKey, P256VerifyingKey) {
        let signing_key = P256SigningKey::random(&mut thread_rng());
        let verifying_key = *signing_key.verifying_key();
        (signing_key, verifying_key)
    }

    pub fn sign(signing_key: &P256SigningKey, msg: &[u8]) -> P256Signature {
        signing_key.sign(msg)
    }

    pub fn verify(verifying_key: &P256VerifyingKey, msg: &[u8], signature: &P256Signature) -> bool {
        verifying_key.verify(msg, signature).is_ok()
    }
}

impl AlgorithmTrait for EcdsaP256 {
    fn name(&self) -> &'static str {
        "ecdsa-p256"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl CryptoAlgorithmTrait for EcdsaP256 {}

impl EcdsaAlgorithmTrait for EcdsaP256 {
    fn curve_name(&self) -> &'static str {
        "P-256"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecdsa_p256() {
        let (sk, vk) = EcdsaP256::generate_keypair();
        let msg = b"test message p256";
        let sig = EcdsaP256::sign(&sk, msg);
        assert!(EcdsaP256::verify(&vk, msg, &sig));
    }
}
