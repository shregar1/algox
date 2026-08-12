use crate::abstraction::AlgorithmTrait;
use super::abstraction::HashingAlgorithmTrait;

const BASE32_CHARS: &[u8] = b"0123456789bcdefghjkmnpqrstuvwxyz";

/// Geohash encoding and decoding utility for spatial coordinate hashing.
pub struct Geohash;

impl Geohash {
    /// Encodes `(lat, lon)` into a geohash string of length `precision`.
    pub fn encode(lat: f64, lon: f64, precision: usize) -> String {
        if precision == 0 {
            return String::new();
        }

        let mut lat_range = (-90.0, 90.0);
        let mut lon_range = (-180.0, 180.0);

        let mut bits = 0u32;
        let mut bit_count = 0;
        let mut hash = String::with_capacity(precision);
        let mut even = true; // start with longitude

        while hash.len() < precision {
            if even {
                let mid = (lon_range.0 + lon_range.1) / 2.0;
                if lon >= mid {
                    bits = (bits << 1) | 1;
                    lon_range.0 = mid;
                } else {
                    bits <<= 1;
                    lon_range.1 = mid;
                }
            } else {
                let mid = (lat_range.0 + lat_range.1) / 2.0;
                if lat >= mid {
                    bits = (bits << 1) | 1;
                    lat_range.0 = mid;
                } else {
                    bits <<= 1;
                    lat_range.1 = mid;
                }
            }
            even = !even;
            bit_count += 1;

            if bit_count == 5 {
                hash.push(BASE32_CHARS[bits as usize] as char);
                bits = 0;
                bit_count = 0;
            }
        }

        hash
    }

    /// Decodes a geohash string back into `(lat, lon)` bounding box center.
    pub fn decode(hash: &str) -> Option<(f64, f64)> {
        if hash.is_empty() {
            return None;
        }

        let mut lat_range = (-90.0, 90.0);
        let mut lon_range = (-180.0, 180.0);
        let mut even = true;

        for ch in hash.chars() {
            let idx = BASE32_CHARS.iter().position(|&c| c as char == ch)?;
            for i in (0..5).rev() {
                let bit = (idx >> i) & 1;
                if even {
                    let mid = (lon_range.0 + lon_range.1) / 2.0;
                    if bit == 1 {
                        lon_range.0 = mid;
                    } else {
                        lon_range.1 = mid;
                    }
                } else {
                    let mid = (lat_range.0 + lat_range.1) / 2.0;
                    if bit == 1 {
                        lat_range.0 = mid;
                    } else {
                        lat_range.1 = mid;
                    }
                }
                even = !even;
            }
        }

        let lat = (lat_range.0 + lat_range.1) / 2.0;
        let lon = (lon_range.0 + lon_range.1) / 2.0;
        Some((lat, lon))
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
