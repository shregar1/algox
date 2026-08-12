use crate::abstraction::AlgorithmTrait;
use super::abstraction::EncodingAlgorithmTrait;

pub struct Hex;

impl Hex {
    pub fn encode(data: &[u8]) -> String {
        hex::encode(data)
    }

    pub fn decode(encoded: &str) -> Result<Vec<u8>, String> {
        hex::decode(encoded).map_err(|e| e.to_string())
    }
}

impl AlgorithmTrait for Hex {
    fn name(&self) -> &'static str {
        "hex"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl EncodingAlgorithmTrait for Hex {
    fn encode(&self, data: &[u8]) -> String {
        Self::encode(data)
    }

    fn decode(&self, encoded: &str) -> Result<Vec<u8>, String> {
        Self::decode(encoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex() {
        let data = b"hello hex";
        let enc = Hex::encode(data);
        let dec = Hex::decode(&enc).unwrap();
        assert_eq!(dec, data);
    }
}
