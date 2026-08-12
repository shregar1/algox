use crate::abstraction::AlgorithmTrait;
use super::abstraction::HashingAlgorithmTrait;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

pub struct Pbkdf2 {
    rounds: u32,
}

impl Pbkdf2 {
    pub fn new(rounds: u32) -> Self {
        Self { rounds }
    }

    pub fn derive_key(&self, password: &[u8], salt: &[u8], output: &mut [u8]) {
        pbkdf2_hmac::<Sha256>(password, salt, self.rounds, output);
    }
}

impl Default for Pbkdf2 {
    fn default() -> Self {
        Self::new(100_000)
    }
}

impl AlgorithmTrait for Pbkdf2 {
    fn name(&self) -> &'static str {
        "pbkdf2"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for Pbkdf2 {
    type Output = [u8; 32];

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        let mut key = [0u8; 32];
        self.derive_key(bytes, b"default_salt", &mut key);
        key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pbkdf2() {
        let algo = Pbkdf2::new(1000);
        let mut key1 = [0u8; 32];
        let mut key2 = [0u8; 32];
        algo.derive_key(b"password", b"salt", &mut key1);
        algo.derive_key(b"password", b"salt", &mut key2);
        assert_eq!(key1, key2);
    }
}
