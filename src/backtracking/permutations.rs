use crate::abstraction::AlgorithmTrait;
use super::abstraction::BacktrackingAlgorithmTrait;

/// Backtracking Permutation Generator.
pub struct Permutations;

impl Permutations {
    /// Generates all distinct permutations of a slice `items`.
    pub fn generate<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
        let mut results = Vec::new();
        let mut current = Vec::new();
        let mut used = vec![false; items.len()];
        Self::backtrack(items, &mut used, &mut current, &mut results);
        results
    }

    fn backtrack<T: Clone>(
        items: &[T],
        used: &mut [bool],
        current: &mut Vec<T>,
        results: &mut Vec<Vec<T>>,
    ) {
        if current.len() == items.len() {
            results.push(current.clone());
            return;
        }

        for i in 0..items.len() {
            if !used[i] {
                used[i] = true;
                current.push(items[i].clone());
                Self::backtrack(items, used, current, results);
                current.pop();
                used[i] = false;
            }
        }
    }
}

impl AlgorithmTrait for Permutations {
    fn name(&self) -> &'static str {
        "permutations"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl BacktrackingAlgorithmTrait for Permutations {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations() {
        let mut p = Permutations;
        assert_eq!(p.name(), "permutations");
        assert_eq!(p.len(), 0);
        p.clear();

        let perms = Permutations::generate(&[1, 2, 3]);
        assert_eq!(perms.len(), 6);
        assert_eq!(Permutations::generate::<i32>(&[]), vec![vec![]]);
        assert_eq!(Permutations::generate(&[42]), vec![vec![42]]);
    }
}
