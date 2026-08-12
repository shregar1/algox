use crate::abstraction::AlgorithmTrait;
use super::abstraction::SearchAlgorithmTrait;

pub struct ExponentialSearch;

impl ExponentialSearch {
    pub fn search<T: Ord>(slice: &[T], target: &T) -> Option<usize> {
        if slice.is_empty() {
            return None;
        }

        if &slice[0] == target {
            return Some(0);
        }

        let mut bound = 1;
        while bound < slice.len() && &slice[bound] <= target {
            bound *= 2;
        }

        let left = bound / 2;
        let right = bound.min(slice.len());

        slice[left..right]
            .binary_search(target)
            .ok()
            .map(|idx| left + idx)
    }
}

impl AlgorithmTrait for ExponentialSearch {
    fn name(&self) -> &'static str {
        "exponential_search"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl<T: Ord> SearchAlgorithmTrait<T> for ExponentialSearch {
    fn search(&self, slice: &[T], target: &T) -> Option<usize> {
        Self::search(slice, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_search() {
        let arr = [2, 4, 6, 8, 10, 12, 14, 16, 18, 20];
        assert_eq!(ExponentialSearch::search(&arr, &14), Some(6));
        assert_eq!(ExponentialSearch::search(&arr, &5), None);
    }
}
