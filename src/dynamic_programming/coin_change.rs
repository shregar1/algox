use crate::abstraction::AlgorithmTrait;
use super::abstraction::DynamicProgrammingTrait;

/// Coin Change — minimum number of coins to make an amount.
pub struct CoinChange;

impl CoinChange {
    /// Returns the minimum number of coins from `coins` that sum to `amount`,
    /// or `None` if it is not possible.
    pub fn min_coins(coins: &[u64], amount: u64) -> Option<u64> {
        let n = (amount + 1) as usize;
        let mut dp = vec![u64::MAX; n];
        dp[0] = 0;
        for a in 1..n {
            for &c in coins {
                if c as usize <= a {
                    let prev = dp[a - c as usize];
                    if prev != u64::MAX {
                        dp[a] = dp[a].min(prev + 1);
                    }
                }
            }
        }
        if dp[amount as usize] == u64::MAX { None } else { Some(dp[amount as usize]) }
    }

    /// Returns one possible set of coins (with repetition) that achieves the minimum.
    pub fn coins_used(coins: &[u64], amount: u64) -> Option<Vec<u64>> {
        let n = (amount + 1) as usize;
        let mut dp = vec![u64::MAX; n];
        let mut last = vec![0u64; n];
        dp[0] = 0;
        for a in 1..n {
            for &c in coins {
                if c as usize <= a {
                    let prev = dp[a - c as usize];
                    if prev != u64::MAX && prev + 1 < dp[a] {
                        dp[a] = prev + 1;
                        last[a] = c;
                    }
                }
            }
        }
        if dp[amount as usize] == u64::MAX {
            return None;
        }
        let mut result = Vec::new();
        let mut rem = amount;
        while rem > 0 {
            let c = last[rem as usize];
            result.push(c);
            rem -= c;
        }
        Some(result)
    }
}

impl AlgorithmTrait for CoinChange {
    fn name(&self) -> &'static str {
        "coin_change"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl DynamicProgrammingTrait for CoinChange {
    fn description(&self) -> &'static str {
        "Coin Change: find the minimum number of coins that sum to a given amount."
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_change_min() {
        assert_eq!(CoinChange::min_coins(&[1, 5, 6, 9], 11), Some(2)); // 5+6
    }

    #[test]
    fn test_coin_change_impossible() {
        assert_eq!(CoinChange::min_coins(&[2], 3), None);
    }

    #[test]
    fn test_coin_change_coins_used() {
        let mut used = CoinChange::coins_used(&[1, 5, 6, 9], 11).unwrap();
        used.sort_unstable();
        assert_eq!(used, vec![5, 6]);
    }
}
