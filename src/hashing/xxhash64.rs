use crate::abstraction::AlgorithmTrait;
use super::abstraction::HashingAlgorithmTrait;

const XX_PRIME_64_1: u64 = 0x9e3779b97f4a7c15;
const XX_PRIME_64_2: u64 = 0x85ebca77c2b2ae63;
const XX_PRIME_64_3: u64 = 0xc2b2ae3d27d4eb4f;
const XX_PRIME_64_5: u64 = 0x85ebca6b;

pub struct XxHash64;

impl XxHash64 {
    pub fn digest(bytes: &[u8], seed: u64) -> u64 {
        let len = bytes.len();
        let mut h: u64;
        let mut i = 0;

        if len >= 32 {
            let mut v1 = seed.wrapping_add(XX_PRIME_64_2).wrapping_add(XX_PRIME_64_1);
            let mut v2 = seed.wrapping_add(XX_PRIME_64_2);
            let mut v3 = seed.wrapping_add(XX_PRIME_64_2.wrapping_mul(2));
            let mut v4 = seed.wrapping_add(XX_PRIME_64_2.wrapping_sub(XX_PRIME_64_1));

            while i + 32 <= len {
                v1 = Self::round64(v1, Self::read_u64(&bytes[i..]));
                v2 = Self::round64(v2, Self::read_u64(&bytes[i + 8..]));
                v3 = Self::round64(v3, Self::read_u64(&bytes[i + 16..]));
                v4 = Self::round64(v4, Self::read_u64(&bytes[i + 24..]));
                i += 32;
            }

            h = v1
                .rotate_left(1)
                .wrapping_add(v2.rotate_left(7))
                .wrapping_add(v3.rotate_left(12))
                .wrapping_add(v4.rotate_left(18));
            h = Self::merge_round64(h, v1);
            h = Self::merge_round64(h, v2);
            h = Self::merge_round64(h, v3);
            h = Self::merge_round64(h, v4);
        } else {
            h = seed.wrapping_add(XX_PRIME_64_5);
        }

        h = h.wrapping_add(len as u64);

        while i + 8 <= len {
            h = Self::merge_round64(h, Self::read_u64(&bytes[i..]).wrapping_mul(XX_PRIME_64_2));
            i += 8;
        }

        while i + 4 <= len {
            h = Self::merge_round64(h, (Self::read_u32(&bytes[i..]) as u64).wrapping_mul(XX_PRIME_64_1));
            i += 4;
        }

        while i < len {
            h = Self::merge_round64(h, (bytes[i] as u64).wrapping_mul(XX_PRIME_64_5));
            i += 1;
        }

        Self::avalanche64(h)
    }

    #[inline]
    fn read_u32(b: &[u8]) -> u32 {
        u32::from_le_bytes([b[0], b[1], b[2], b[3]])
    }

    #[inline]
    fn read_u64(b: &[u8]) -> u64 {
        u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
    }

    #[inline]
    fn round64(acc: u64, input: u64) -> u64 {
        let acc = acc.wrapping_add(input.wrapping_mul(XX_PRIME_64_2));
        acc.rotate_left(31).wrapping_mul(XX_PRIME_64_1)
    }

    #[inline]
    fn merge_round64(acc: u64, val: u64) -> u64 {
        let acc = acc ^ Self::round64(0, val);
        acc.rotate_left(1).wrapping_add(XX_PRIME_64_3).wrapping_mul(5)
    }

    #[inline]
    fn avalanche64(mut h: u64) -> u64 {
        h ^= h >> 33;
        h = h.wrapping_mul(XX_PRIME_64_2);
        h ^= h >> 29;
        h = h.wrapping_mul(XX_PRIME_64_3);
        h ^= h >> 32;
        h
    }
}

impl AlgorithmTrait for XxHash64 {
    fn name(&self) -> &'static str {
        "xxhash64"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for XxHash64 {
    type Output = u64;

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        Self::digest(bytes, 0)
    }
}

pub fn xxhash64(bytes: &[u8], seed: u64) -> u64 {
    XxHash64::digest(bytes, seed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xxhash64_deterministic() {
        assert_eq!(xxhash64(b"hello", 0), xxhash64(b"hello", 0));
    }

    #[test]
    fn test_xxhash64_seed_changes() {
        assert_ne!(xxhash64(b"hello", 0), xxhash64(b"hello", 1));
    }

    #[test]
    fn test_xxhash64_empty() {
        let h = xxhash64(b"", 0);
        assert_ne!(h, 0);
    }

    #[test]
    fn test_xxhash64_long_input() {
        let long = vec![0xab; 256];
        let h = xxhash64(&long, 42);
        assert_ne!(h, 0);
    }
}
