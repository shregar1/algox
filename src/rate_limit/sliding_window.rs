use crate::abstraction::AlgorithmTrait;
use super::abstraction::RateLimitAlgorithmTrait;
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct WindowState {
    timestamps: Vec<Instant>,
}

pub struct SlidingWindow {
    max_requests: u64,
    window_duration: Duration,
    windows: HashMap<String, WindowState>,
}

impl SlidingWindow {
    pub fn new(max_requests: u64, window_duration: Duration) -> Self {
        Self {
            max_requests,
            window_duration,
            windows: HashMap::new(),
        }
    }
}

impl AlgorithmTrait for SlidingWindow {
    fn name(&self) -> &'static str {
        "sliding_window"
    }

    fn len(&self) -> usize {
        self.windows.len()
    }

    fn clear(&mut self) {
        self.windows.clear();
    }
}

impl RateLimitAlgorithmTrait for SlidingWindow {
    fn check_and_consume(&mut self, key: &str, cost: u64) -> bool {
        let now = Instant::now();
        let cutoff = now.checked_sub(self.window_duration).unwrap_or(now);

        let state = self.windows.entry(key.to_string()).or_insert_with(|| WindowState {
            timestamps: Vec::new(),
        });

        state.timestamps.retain(|&ts| ts > cutoff);

        if (state.timestamps.len() as u64) + cost <= self.max_requests {
            for _ in 0..cost {
                state.timestamps.push(now);
            }
            true
        } else {
            false
        }
    }

    fn reset_key(&mut self, key: &str) {
        self.windows.remove(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sliding_window() {
        let mut sw = SlidingWindow::new(5, Duration::from_secs(60));
        assert!(sw.check_and_consume("user1", 3));
        assert!(sw.check_and_consume("user1", 2));
        assert!(!sw.check_and_consume("user1", 1));
    }
}
