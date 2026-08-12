use crate::abstraction::AlgorithmTrait;
use super::abstraction::HashingAlgorithmTrait;

const XX_PRIME_32_1: u32 = 0x9e3779b1;
const XX_PRIME_32_2: u32 = 0x85ebca77;
const XX_PRIME_32_3: u32 = 0xc2b2ae3d;
const XX_PRIME_32_4: u32 = 0x27d4eb2f;
const XX_PRIME_32_5: u32 = 0x165667b1;

pub struct XxHash32;

impl XxHash32 {
    pub fn digest(bytes: &[u8], seed: u32) -> u32 {
        let mut h: u32;
        let len = bytes.len();
        let mut i = 0;

        if len >= 16 {
            let mut v1 = seed.wrapping_add(XX_PRIME_32_1).wrapping_add(XX_PRIME_32_2);
            let mut v2 = seed.wrapping_add(XX_PRIME_32_2);
            let mut v3 = seed;
            let mut v4 = seed.wrapping_sub(XX_PRIME_32_1);

            while i + 16 <= len {
                v1 = Self::round32(v1, Self::read_u32(&bytes[i..]));
                v2 = Self::round32(v2, Self::read_u32(&bytes[i + 4..]));
                v3 = Self::round32(v3, Self::read_u32(&bytes[i + 8..]));
                v4 = Self::round32(v4, Self::read_u32(&bytes[i + 12..]));
                i += 16;
            }

            h = v1
                .rotate_left(1)
                .wrapping_add(v2.rotate_left(7))
                .wrapping_add(v3.rotate_left(12))
                .wrapping_add(v4.rotate_left(18));
            h = Self::merge_round32(h, v1);
            h = Self::merge_round32(h, v2);
            h = Self::merge_round32(h, v3);
            h = Self::merge_round32(h, v4);
        } else {
            h = seed.wrapping_add(XX_PRIME_32_5);
        }

        h = h.wrapping_add(len as u32);

        while i + 4 <= len {
            h = h.wrapping_add(Self::read_u32(&bytes[i..]).wrapping_mul(XX_PRIME_32_3));
            h = h.rotate_left(17).wrapping_mul(XX_PRIME_32_4);
            i += 4;
        }

        while i < len {
            h = h.wrapping_add((bytes[i] as u32).wrapping_mul(XX_PRIME_32_5));
            h = h.rotate_left(11).wrapping_mul(XX_PRIME_32_1);
            i += 1;
        }

        Self::avalanche32(h)
    }

    #[inline]
    fn read_u32(b: &[u8]) -> u32 {
        u32::from_le_bytes([b[0], b[1], b[2], b[3]])
    }

    #[inline]
    fn round32(acc: u32, input: u32) -> u32 {
        let acc = acc.wrapping_add(input.wrapping_mul(XX_PRIME_32_2));
        acc.rotate_left(13).wrapping_mul(XX_PRIME_32_1)
    }

    #[inline]
    fn merge_round32(acc: u32, val: u32) -> u32 {
        let acc = acc ^ Self::round32(0, val);
        acc.rotate_left(1).wrapping_mul(5).wrapping_add(XX_PRIME_32_4)
    }

    #[inline]
    fn avalanche32(mut h: u32) -> u32 {
        h ^= h >> 15;
        h = h.wrapping_mul(XX_PRIME_32_2);
        h ^= h >> 13;
        h = h.wrapping_mul(XX_PRIME_32_3);
        h ^= h >> 16;
        h
    }
}

impl AlgorithmTrait for XxHash32 {
    fn name(&self) -> &'static str {
        "xxhash32"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl HashingAlgorithmTrait for XxHash32 {
    type Output = u32;

    fn digest_bytes(&self, bytes: &[u8]) -> Self::Output {
        Self::digest(bytes, 0)
    }
}

pub fn xxhash32(bytes: &[u8], seed: u32) -> u32 {
    XxHash32::digest(bytes, seed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xxhash32_deterministic() {
        assert_eq!(xxhash32(b"hello", 0), xxhash32(b"hello", 0));
    }

    #[test]
    fn test_xxhash32_seed_changes() {
        assert_ne!(xxhash32(b"hello", 0), xxhash32(b"hello", 1));
    }

    #[test]
    fn test_xxhash32_empty() {
        let h = xxhash32(b"", 0);
        assert_ne!(h, 0);
    }

    #[test]
    fn test_xxhash32_aligned() {
        let aligned = b"0123456789abcdef";
        let h1 = xxhash32(aligned, 0);
        let h2 = xxhash32(aligned, 0);
        assert_eq!(h1, h2);
    }
}
