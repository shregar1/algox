use crate::abstraction::AlgorithmTrait;
use super::abstraction::SortingAlgorithmTrait;

pub struct InsertionSort;

impl InsertionSort {
    pub fn sort<T: Ord>(slice: &mut [T]) {
        for i in 1..slice.len() {
            let mut j = i;
            while j > 0 && slice[j] < slice[j - 1] {
                slice.swap(j, j - 1);
                j -= 1;
            }
        }
    }
}

impl AlgorithmTrait for InsertionSort {
    fn name(&self) -> &'static str {
        "insertion_sort"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl<T: Ord> SortingAlgorithmTrait<T> for InsertionSort {
    fn sort(&self, slice: &mut [T]) {
        Self::sort(slice);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut arr = [5, 2, 8, 1, 9, 4];
        InsertionSort::sort(&mut arr);
        assert_eq!(arr, [1, 2, 4, 5, 8, 9]);
    }
}
