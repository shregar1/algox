use crate::abstraction::AlgorithmTrait;
use super::abstraction::CryptoAlgorithmTrait;
use x25519_dalek::{StaticSecret, PublicKey};
use rand::thread_rng;

pub struct X25519;

impl X25519 {
    pub fn generate_keypair() -> (StaticSecret, PublicKey) {
        let secret = StaticSecret::random_from_rng(thread_rng());
        let public = PublicKey::from(&secret);
        (secret, public)
    }

    pub fn diffie_hellman(secret: &StaticSecret, peer_public: &PublicKey) -> [u8; 32] {
        *secret.diffie_hellman(peer_public).as_bytes()
    }
}

impl AlgorithmTrait for X25519 {
    fn name(&self) -> &'static str {
        "x25519"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl CryptoAlgorithmTrait for X25519 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x25519() {
        let (alice_secret, alice_public) = X25519::generate_keypair();
        let (bob_secret, bob_public) = X25519::generate_keypair();

        let alice_shared = X25519::diffie_hellman(&alice_secret, &bob_public);
        let bob_shared = X25519::diffie_hellman(&bob_secret, &alice_public);

        assert_eq!(alice_shared, bob_shared);
    }
}
