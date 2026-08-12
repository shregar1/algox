use crate::abstraction::AlgorithmTrait;
use super::abstraction::SortingAlgorithmTrait;

pub struct TimSort;

impl TimSort {
    pub fn sort<T: Ord>(slice: &mut [T]) {
        slice.sort();
    }
}

impl AlgorithmTrait for TimSort {
    fn name(&self) -> &'static str {
        "tim_sort"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl<T: Ord> SortingAlgorithmTrait<T> for TimSort {
    fn sort(&self, slice: &mut [T]) {
        Self::sort(slice);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tim_sort() {
        let mut arr = [5, 2, 8, 1, 9, 4];
        TimSort::sort(&mut arr);
        assert_eq!(arr, [1, 2, 4, 5, 8, 9]);
    }
}
