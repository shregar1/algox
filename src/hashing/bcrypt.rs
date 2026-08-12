use crate::abstraction::AlgorithmTrait;
use super::abstraction::HashingAlgorithmTrait;

pub struct Bcrypt {
    cost: u32,
}

impl Bcrypt {
    pub fn new(cost: u32) -> Self {
        Self { cost }
    }

    pub fn hash(&self, data: &str) -> Result<String, String> {
        bcrypt::hash(data, self.cost).map_err(|e| e.to_string())
    }

    pub fn verify(&self, data: &str, hash: &str) -> Result<bool, String> {
        bcrypt::verify(data, hash).map_err(|e| e.to_string())
    }
}

impl Default for Bcrypt {
    fn default() -> Self {
        Self::new(bcrypt::DEFAULT_COST)
    }
}

impl AlgorithmTrait for Bcrypt {
    fn name(&self) -> &'static str {
        "bcrypt"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for Bcrypt {
    type Output = String;

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        let pwd = String::from_utf8_lossy(bytes);
        self.hash(&pwd).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bcrypt() {
        let algo = Bcrypt::new(4);
        let hash = algo.hash("secret").unwrap();
        assert!(algo.verify("secret", &hash).unwrap());
        assert!(!algo.verify("wrong", &hash).unwrap());
    }
}
