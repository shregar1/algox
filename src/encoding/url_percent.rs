use crate::abstraction::AlgorithmTrait;
use super::abstraction::EncodingAlgorithmTrait;
use percent_encoding::{percent_decode_str, utf8_percent_encode, NON_ALPHANUMERIC};

pub struct UrlPercent;

impl UrlPercent {
    pub fn encode(data: &[u8]) -> String {
        utf8_percent_encode(&String::from_utf8_lossy(data), NON_ALPHANUMERIC).to_string()
    }

    pub fn decode(encoded: &str) -> Result<Vec<u8>, String> {
        percent_decode_str(encoded)
            .decode_utf8()
            .map(|cow| cow.as_bytes().to_vec())
            .map_err(|e| e.to_string())
    }
}

impl AlgorithmTrait for UrlPercent {
    fn name(&self) -> &'static str {
        "url_percent"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl EncodingAlgorithmTrait for UrlPercent {
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
    fn test_url_percent() {
        let data = b"hello world!";
        let enc = UrlPercent::encode(data);
        let dec = UrlPercent::decode(&enc).unwrap();
        assert_eq!(dec, data);
    }
}
