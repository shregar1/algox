use crate::abstraction::AlgorithmTrait;
use super::super::abstraction::CryptoAlgorithmTrait;
use super::abstraction::EcdsaAlgorithmTrait;
use p384::ecdsa::{SigningKey as P384SigningKey, VerifyingKey as P384VerifyingKey, Signature as P384Signature, signature::Signer, signature::Verifier};
use rand::thread_rng;

pub struct EcdsaP384;

impl EcdsaP384 {
    pub fn generate_keypair() -> (P384SigningKey, P384VerifyingKey) {
        let signing_key = P384SigningKey::random(&mut thread_rng());
        let verifying_key = *signing_key.verifying_key();
        (signing_key, verifying_key)
    }

    pub fn sign(signing_key: &P384SigningKey, msg: &[u8]) -> P384Signature {
        signing_key.sign(msg)
    }

    pub fn verify(verifying_key: &P384VerifyingKey, msg: &[u8], signature: &P384Signature) -> bool {
        verifying_key.verify(msg, signature).is_ok()
    }
}

impl AlgorithmTrait for EcdsaP384 {
    fn name(&self) -> &'static str {
        "ecdsa-p384"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl CryptoAlgorithmTrait for EcdsaP384 {}

impl EcdsaAlgorithmTrait for EcdsaP384 {
    fn curve_name(&self) -> &'static str {
        "P-384"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecdsa_p384() {
        let (sk, vk) = EcdsaP384::generate_keypair();
        let msg = b"test message p384";
        let sig = EcdsaP384::sign(&sk, msg);
        assert!(EcdsaP384::verify(&vk, msg, &sig));
    }
}
