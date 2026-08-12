use crate::abstraction::AlgorithmTrait;
use super::abstraction::{SchedulingAlgorithmTrait, Job};

/// Weighted Job Scheduling — DP: maximise total weight of non-overlapping jobs.
pub struct WeightedJobScheduling;

impl WeightedJobScheduling {
    /// Returns the maximum total weight achievable with non-overlapping jobs.
    pub fn max_weight(jobs: &[Job]) -> i64 {
        if jobs.is_empty() { return 0; }
        let n = jobs.len();
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by_key(|&i| jobs[i].finish);
        let sorted: Vec<&Job> = order.iter().map(|&i| &jobs[i]).collect();

        let mut dp = vec![0i64; n + 1];
        for i in 1..=n {
            let job = sorted[i - 1];
            // Binary search for latest job that finishes <= job.start
            let p = Self::latest_non_conflict(&sorted, i - 1, job.start);
            dp[i] = dp[i - 1].max(dp[p] + job.weight);
        }
        dp[n]
    }

    fn latest_non_conflict(sorted: &[&Job], idx: usize, start: i64) -> usize {
        let (mut lo, mut hi) = (0usize, idx);
        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            if sorted[mid - 1].finish <= start {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }
        lo
    }
}

impl AlgorithmTrait for WeightedJobScheduling {
    fn name(&self) -> &'static str {
        "weighted_job_scheduling"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl SchedulingAlgorithmTrait for WeightedJobScheduling {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weighted_job_scheduling() {
        let jobs = vec![
            Job::new(1, 2, 50),
            Job::new(3, 5, 20),
            Job::new(6, 19, 100),
            Job::new(2, 100, 200),
        ];
        assert_eq!(WeightedJobScheduling::max_weight(&jobs), 250); // job0 + job2
    }

    #[test]
    fn test_no_overlap() {
        let jobs = vec![
            Job::new(0, 1, 10),
            Job::new(1, 2, 10),
            Job::new(2, 3, 10),
        ];
        assert_eq!(WeightedJobScheduling::max_weight(&jobs), 30);
    }
}
