use crate::abstraction::AlgorithmTrait;
use super::abstraction::SearchAlgorithmTrait;

pub struct LinearSearch;

impl LinearSearch {
    pub fn search<T: PartialEq>(slice: &[T], target: &T) -> Option<usize> {
        slice.iter().position(|x| x == target)
    }
}

impl AlgorithmTrait for LinearSearch {
    fn name(&self) -> &'static str {
        "linear_search"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl<T: PartialEq> SearchAlgorithmTrait<T> for LinearSearch {
    fn search(&self, slice: &[T], target: &T) -> Option<usize> {
        Self::search(slice, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        let arr = [10, 20, 30, 40];
        assert_eq!(LinearSearch::search(&arr, &30), Some(2));
        assert_eq!(LinearSearch::search(&arr, &50), None);
    }
}
