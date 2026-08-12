use crate::abstraction::AlgorithmTrait;
use super::abstraction::MathAlgorithmTrait;

/// Sieve of Eratosthenes — generate all primes up to N.
pub struct Sieve;

impl Sieve {
    /// Returns all primes ≤ `limit`.
    pub fn primes_up_to(limit: usize) -> Vec<usize> {
        if limit < 2 {
            return Vec::new();
        }
        let mut is_prime = vec![true; limit + 1];
        is_prime[0] = false;
        is_prime[1] = false;
        let mut i = 2;
        while i * i <= limit {
            if is_prime[i] {
                let mut j = i * i;
                while j <= limit {
                    is_prime[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
        is_prime.iter().enumerate()
            .filter_map(|(n, &p)| if p { Some(n) } else { None })
            .collect()
    }

    /// Returns `true` if `n` is prime (trial-division, good for occasional checks).
    pub fn is_prime(n: u64) -> bool {
        if n < 2 { return false; }
        if n == 2 { return true; }
        if n % 2 == 0 { return false; }
        let mut i = 3u64;
        while i * i <= n {
            if n % i == 0 { return false; }
            i += 2;
        }
        true
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
