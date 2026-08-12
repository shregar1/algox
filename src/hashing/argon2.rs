use crate::abstraction::AlgorithmTrait;
use super::abstraction::HashingAlgorithmTrait;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2 as Argon2Hasher,
};

pub struct Argon2;

impl Argon2 {
    pub fn new() -> Self {
        Self
    }

    pub fn hash(&self, data: &str, salt: &str) -> Result<String, String> {
        let salt = SaltString::encode_b64(salt.as_bytes()).map_err(|e| e.to_string())?;
        let argon2 = Argon2Hasher::default();
        argon2
            .hash_password(data.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|e| e.to_string())
    }

    pub fn verify(&self, data: &str, hash: &str) -> Result<bool, String> {
        let parsed_hash = PasswordHash::new(hash).map_err(|e| e.to_string())?;
        Ok(Argon2Hasher::default()
            .verify_password(data.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

impl Default for Argon2 {
    fn default() -> Self {
        Self::new()
    }
}

impl AlgorithmTrait for Argon2 {
    fn name(&self) -> &'static str {
        "argon2"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for Argon2 {
    type Output = String;

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        let pwd = String::from_utf8_lossy(bytes);
        self.hash(&pwd, "default_salt_string_123").unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_argon2() {
        let algo = Argon2::new();
        let hash = algo.hash("secret", "salt1234567890").unwrap();
        assert!(algo.verify("secret", &hash).unwrap());
        assert!(!algo.verify("wrong", &hash).unwrap());
    }
}
