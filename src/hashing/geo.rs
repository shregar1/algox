use crate::abstraction::AlgorithmTrait;
use super::abstraction::HashingAlgorithmTrait;
use geohash::{Coord, decode as gh_decode, encode as gh_encode};

/// Geohash encoding and decoding utility leveraging the production-grade `geohash` crate.
pub struct Geohash;

impl Geohash {
    /// Encodes `(lat, lon)` into a geohash string of length `precision`.
    pub fn encode(lat: f64, lon: f64, precision: usize) -> String {
        if precision == 0 {
            return String::new();
        }
        let coord = Coord { x: lon, y: lat };
        gh_encode(coord, precision).unwrap_or_default()
    }

    /// Decodes a geohash string back into `(lat, lon)` bounding box center.
    pub fn decode(hash: &str) -> Option<(f64, f64)> {
        if hash.is_empty() {
            return None;
        }
        let (coord, _, _) = gh_decode(hash).ok()?;
        Some((coord.y, coord.x))
    }
}

impl AlgorithmTrait for Geohash {
    fn name(&self) -> &'static str {
        "geohash"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for Geohash {
    type Output = String;

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        if let Ok(s) = std::str::from_utf8(bytes) {
            if let Some((lat_str, lon_str)) = s.split_once(',') {
                if let (Ok(lat), Ok(lon)) = (lat_str.trim().parse::<f64>(), lon_str.trim().parse::<f64>()) {
                    return Self::encode(lat, lon, 6);
                }
            }
        }
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geohash_encode_decode() {
        let lat = 42.6;
        let lon = -5.6;
        let hash = Geohash::encode(lat, lon, 5);
        assert_eq!(hash, "ezs42");

        let (d_lat, d_lon) = Geohash::decode(&hash).unwrap();
        assert!((d_lat - lat).abs() < 0.1);
        assert!((d_lon - lon).abs() < 0.1);
    }
}
