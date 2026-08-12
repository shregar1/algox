use crate::abstraction::AlgorithmTrait;
use super::abstraction::MathAlgorithmTrait;

/// GCD and LCM using the Euclidean algorithm.
pub struct Gcd;

impl Gcd {
    /// Returns the greatest common divisor of `a` and `b`.
    pub fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }

    /// Returns the least common multiple of `a` and `b`.
    pub fn lcm(a: u64, b: u64) -> u64 {
        if a == 0 || b == 0 {
            return 0;
        }
        a / Self::gcd(a, b) * b
    }

    /// Extended Euclidean: returns (gcd, x, y) such that a*x + b*y = gcd.
    pub fn extended(a: i64, b: i64) -> (i64, i64, i64) {
        if b == 0 {
            return (a, 1, 0);
        }
        let (g, x1, y1) = Self::extended(b, a % b);
        (g, y1, x1 - (a / b) * y1)
    }
}

impl AlgorithmTrait for Gcd {
    fn name(&self) -> &'static str {
        "gcd"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl MathAlgorithmTrait for Gcd {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(Gcd::gcd(48, 18), 6);
        assert_eq!(Gcd::gcd(0, 5), 5);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(Gcd::lcm(4, 6), 12);
        assert_eq!(Gcd::lcm(0, 5), 0);
    }

    #[test]
    fn test_extended_gcd() {
        let (g, x, y) = Gcd::extended(35, 15);
        assert_eq!(g, 5);
        assert_eq!(35 * x + 15 * y, g);
    }
}
