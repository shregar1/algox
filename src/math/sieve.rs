use crate::abstraction::AlgorithmTrait;
use super::abstraction::MathAlgorithmTrait;

/// Sieve of Eratosthenes and Primality Testing backed by the production-grade `primal` crate.
pub struct Sieve;

impl Sieve {
    /// Returns all primes ≤ `limit`.
    pub fn primes_up_to(limit: usize) -> Vec<usize> {
        let sieve = primal::Sieve::new(limit);
        sieve.primes_from(0).take_while(|&p| p <= limit).collect()
    }

    /// Returns `true` if `n` is prime.
    pub fn is_prime(n: u64) -> bool {
        primal::is_prime(n)
    }
}

impl AlgorithmTrait for Sieve {
    fn name(&self) -> &'static str {
        "sieve"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl MathAlgorithmTrait for Sieve {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve() {
        assert_eq!(Sieve::primes_up_to(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }

    #[test]
    fn test_is_prime() {
        assert!(Sieve::is_prime(97));
        assert!(!Sieve::is_prime(100));
        assert!(!Sieve::is_prime(1));
    }
}
