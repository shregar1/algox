use crate::abstraction::AlgorithmTrait;
use super::abstraction::{SchedulingAlgorithmTrait, Job};

/// Activity Selection — greedy: maximise the number of non-overlapping jobs.
pub struct ActivitySelection;

impl ActivitySelection {
    /// Returns the indices (0-based) of selected jobs in a maximum compatible subset.
    /// Jobs are sorted by finish time before selection.
    pub fn select(jobs: &[Job]) -> Vec<usize> {
        if jobs.is_empty() { return Vec::new(); }
        // pair (finish, original_index) and sort by finish
        let mut order: Vec<usize> = (0..jobs.len()).collect();
        order.sort_by_key(|&i| jobs[i].finish);

        let mut selected = Vec::new();
        let mut last_finish = i64::MIN;
        for idx in order {
            if jobs[idx].start >= last_finish {
                selected.push(idx);
                last_finish = jobs[idx].finish;
            }
        }
        selected
    }
}

impl AlgorithmTrait for ActivitySelection {
    fn name(&self) -> &'static str {
        "activity_selection"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl SchedulingAlgorithmTrait for ActivitySelection {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_activity_selection() {
        let jobs = vec![
            Job::new(1, 4, 1),
            Job::new(3, 5, 1),
            Job::new(0, 6, 1),
            Job::new(5, 7, 1),
            Job::new(3, 9, 1),
            Job::new(5, 9, 1),
            Job::new(6, 10, 1),
            Job::new(8, 11, 1),
            Job::new(8, 12, 1),
            Job::new(2, 14, 1),
            Job::new(12, 16, 1),
        ];
        let selected = ActivitySelection::select(&jobs);
        // Classic example: maximum 4 activities
        assert_eq!(selected.len(), 4);
    }
}
