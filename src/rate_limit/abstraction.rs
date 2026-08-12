use crate::abstraction::AlgorithmTrait;

/// Trait specific to rate limiting algorithms.
pub trait RateLimitAlgorithmTrait: AlgorithmTrait {
    /// Check if a request is allowed for a key. Returns true if permitted.
    fn check_and_consume(&mut self, key: &str, cost: u64) -> bool;

    /// Reset or clear the rate limit state for a key.
    fn reset_key(&mut self, key: &str);
}
