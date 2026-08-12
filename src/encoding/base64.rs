use crate::abstraction::AlgorithmTrait;
use super::abstraction::EncodingAlgorithmTrait;
use base64ct::{Base64 as B64, Base64UrlUnpadded, Encoding};

pub struct Base64;

impl Base64 {
    pub fn encode(data: &[u8]) -> String {
        B64::encode_string(data)
    }

    pub fn decode(encoded: &str) -> Result<Vec<u8>, String> {
        B64::decode_vec(encoded).map_err(|e| e.to_string())
    }
}

impl AlgorithmTrait for Base64 {
    fn name(&self) -> &'static str {
        "base64"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl EncodingAlgorithmTrait for Base64 {
    fn encode(&self, data: &[u8]) -> String {
        Self::encode(data)
    }

    fn decode(&self, encoded: &str) -> Result<Vec<u8>, String> {
        Self::decode(encoded)
    }
}

pub struct Base64Url;

impl Base64Url {
    pub fn encode(data: &[u8]) -> String {
        Base64UrlUnpadded::encode_string(data)
    }

    pub fn decode(encoded: &str) -> Result<Vec<u8>, String> {
        Base64UrlUnpadded::decode_vec(encoded).map_err(|e| e.to_string())
    }
}

impl AlgorithmTrait for Base64Url {
    fn name(&self) -> &'static str {
        "base64_url"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl EncodingAlgorithmTrait for Base64Url {
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
    fn test_base64() {
        let data = b"hello base64";
        let enc = Base64::encode(data);
        let dec = Base64::decode(&enc).unwrap();
        assert_eq!(dec, data);

        let enc_url = Base64Url::encode(data);
        let dec_url = Base64Url::decode(&enc_url).unwrap();
        assert_eq!(dec_url, data);
    }
}
