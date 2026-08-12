use crate::abstraction::AlgorithmTrait;
use super::abstraction::SearchAlgorithmTrait;

pub struct BinarySearch;

impl BinarySearch {
    pub fn search<T: Ord>(slice: &[T], target: &T) -> Option<usize> {
        slice.binary_search(target).ok()
    }
}

impl AlgorithmTrait for BinarySearch {
    fn name(&self) -> &'static str {
        "binary_search"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl<T: Ord> SearchAlgorithmTrait<T> for BinarySearch {
    fn search(&self, slice: &[T], target: &T) -> Option<usize> {
        Self::search(slice, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let arr = [1, 3, 5, 7, 9, 11];
        assert_eq!(BinarySearch::search(&arr, &7), Some(3));
        assert_eq!(BinarySearch::search(&arr, &4), None);
    }
}
