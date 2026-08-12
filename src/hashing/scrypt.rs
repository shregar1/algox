use crate::abstraction::AlgorithmTrait;
use super::abstraction::HashingAlgorithmTrait;
use scrypt::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt as ScryptHasher, Params as ScryptParams,
};

pub struct Scrypt {
    params: ScryptParams,
}

impl Scrypt {
    pub fn new() -> Self {
        Self {
            params: ScryptParams::new(15, 8, 1, 32).expect("valid default scrypt params"),
        }
    }

    pub fn with_params(log_n: u8, r: u32, p: u32) -> Result<Self, String> {
        let params = ScryptParams::new(log_n, r, p, 32).map_err(|e| e.to_string())?;
        Ok(Self { params })
    }

    pub fn hash_password(&self, password: &str, salt: &str) -> Result<String, String> {
        let salt = SaltString::encode_b64(salt.as_bytes()).map_err(|e| e.to_string())?;
        let scrypt = ScryptHasher;
        scrypt
            .hash_password_customized(password.as_bytes(), None, None, self.params, &salt)
            .map(|h| h.to_string())
            .map_err(|e| e.to_string())
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, String> {
        let parsed_hash = PasswordHash::new(hash).map_err(|e| e.to_string())?;
        Ok(ScryptHasher
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

impl Default for Scrypt {
    fn default() -> Self {
        Self::new()
    }
}

impl AlgorithmTrait for Scrypt {
    fn name(&self) -> &'static str {
        "scrypt"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for Scrypt {
    type Output = String;

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        let pwd = String::from_utf8_lossy(bytes);
        self.hash_password(&pwd, "default_salt_string_123").unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scrypt() {
        let algo = Scrypt::with_params(10, 8, 1).unwrap();
        let hash = algo.hash_password("secret", "salt1234567890").unwrap();
        assert!(algo.verify_password("secret", &hash).unwrap());
        assert!(!algo.verify_password("wrong", &hash).unwrap());
    }
}
