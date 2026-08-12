use crate::abstraction::AlgorithmTrait;
use super::abstraction::MathAlgorithmTrait;

/// Modular arithmetic utilities — fast modular exponentiation and inverse.
pub struct ModArith;

impl ModArith {
    /// Returns `(base ^ exp) % modulus` using fast binary exponentiation.
    pub fn pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
        if modulus == 1 { return 0; }
        let mut result = 1u64;
        base %= modulus;
        while exp > 0 {
            if exp & 1 == 1 {
                result = result.wrapping_mul(base) % modulus;
            }
            exp >>= 1;
            base = base.wrapping_mul(base) % modulus;
        }
        result
    }

    /// Returns the modular inverse of `a` modulo a prime `m` using Fermat's little theorem.
    /// Returns `None` if `a == 0`.
    pub fn inv_prime_mod(a: u64, m: u64) -> Option<u64> {
        if a == 0 { return None; }
        Some(Self::pow(a, m - 2, m))
    }

    /// Returns `(a * b) % m` without overflow using 128-bit intermediate.
    pub fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
        ((a as u128 * b as u128) % m as u128) as u64
    }
}

impl AlgorithmTrait for ModArith {
    fn name(&self) -> &'static str {
        "mod_arith"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl MathAlgorithmTrait for ModArith {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow_mod() {
        assert_eq!(ModArith::pow(2, 10, 1000), 24);
        assert_eq!(ModArith::pow(3, 0, 7), 1);
        assert_eq!(ModArith::pow(0, 5, 7), 0);
    }

    #[test]
    fn test_inv_prime_mod() {
        let inv = ModArith::inv_prime_mod(3, 7).unwrap();
        assert_eq!((3 * inv) % 7, 1);
    }

    #[test]
    fn test_mul_mod() {
        assert_eq!(ModArith::mul_mod(u64::MAX / 2, 2, 7), (u64::MAX - 1) % 7);
    }
}
