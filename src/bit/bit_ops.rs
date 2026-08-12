use crate::abstraction::AlgorithmTrait;
use super::abstraction::BitAlgorithmTrait;

/// Common bit-manipulation operations.
pub struct BitOps;

impl BitOps {
    /// Returns the number of set bits (popcount / Hamming weight).
    pub fn popcount(mut n: u64) -> u32 {
        // Brian Kernighan's algorithm
        let mut count = 0;
        while n != 0 {
            n &= n - 1;
            count += 1;
        }
        count
    }

    /// Returns the Hamming distance between `a` and `b` (number of differing bits).
    pub fn hamming_distance(a: u64, b: u64) -> u32 {
        Self::popcount(a ^ b)
    }

    /// Returns the smallest power of 2 ≥ `n`. Returns 1 for n == 0.
    pub fn next_power_of_two(n: u64) -> u64 {
        if n <= 1 { return 1; }
        let mut p = 1u64;
        while p < n {
            p <<= 1;
        }
        p
    }

    /// Reverses the bits of a u64.
    pub fn reverse_bits(mut n: u64) -> u64 {
        let mut result = 0u64;
        for _ in 0..64 {
            result = (result << 1) | (n & 1);
            n >>= 1;
        }
        result
    }

    /// Returns `true` if `n` is a power of two (and n > 0).
    pub fn is_power_of_two(n: u64) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }

    /// Returns the position (0-indexed from LSB) of the lowest set bit, or None if n == 0.
    pub fn lowest_set_bit_pos(n: u64) -> Option<u32> {
        if n == 0 { return None; }
        Some(n.trailing_zeros())
    }

    /// Returns `n` with the lowest set bit cleared.
    pub fn clear_lowest_bit(n: u64) -> u64 {
        n & n.wrapping_sub(1)
    }
}

impl AlgorithmTrait for BitOps {
    fn name(&self) -> &'static str {
        "bit_ops"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl BitAlgorithmTrait for BitOps {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_popcount() {
        assert_eq!(BitOps::popcount(0b1011), 3);
        assert_eq!(BitOps::popcount(0), 0);
        assert_eq!(BitOps::popcount(u64::MAX), 64);
    }

    #[test]
    fn test_hamming_distance() {
        assert_eq!(BitOps::hamming_distance(0b1011, 0b1001), 1);
        assert_eq!(BitOps::hamming_distance(7, 7), 0);
    }

    #[test]
    fn test_next_power_of_two() {
        assert_eq!(BitOps::next_power_of_two(0), 1);
        assert_eq!(BitOps::next_power_of_two(1), 1);
        assert_eq!(BitOps::next_power_of_two(5), 8);
        assert_eq!(BitOps::next_power_of_two(16), 16);
    }

    #[test]
    fn test_reverse_bits() {
        assert_eq!(BitOps::reverse_bits(1u64), 1u64 << 63);
    }

    #[test]
    fn test_is_power_of_two() {
        assert!(BitOps::is_power_of_two(8));
        assert!(!BitOps::is_power_of_two(6));
        assert!(!BitOps::is_power_of_two(0));
    }

    #[test]
    fn test_lowest_set_bit() {
        assert_eq!(BitOps::lowest_set_bit_pos(0b1100), Some(2));
        assert_eq!(BitOps::lowest_set_bit_pos(0), None);
    }

    #[test]
    fn test_clear_lowest_bit() {
        assert_eq!(BitOps::clear_lowest_bit(0b1100), 0b1000);
    }
}
