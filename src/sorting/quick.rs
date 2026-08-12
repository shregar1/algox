use crate::abstraction::AlgorithmTrait;
use super::abstraction::SortingAlgorithmTrait;

pub struct QuickSort;

impl QuickSort {
    pub fn sort<T: Ord>(slice: &mut [T]) {
        if slice.len() <= 1 {
            return;
        }
        let pivot_idx = Self::partition(slice);
        Self::sort(&mut slice[..pivot_idx]);
        Self::sort(&mut slice[pivot_idx + 1..]);
    }

    fn partition<T: Ord>(slice: &mut [T]) -> usize {
        let len = slice.len();
        let pivot_idx = len / 2;
        slice.swap(pivot_idx, len - 1);

        let mut store_idx = 0;
        for i in 0..len - 1 {
            if slice[i] < slice[len - 1] {
                slice.swap(i, store_idx);
                store_idx += 1;
            }
        }
        slice.swap(store_idx, len - 1);
        store_idx
    }
}

impl AlgorithmTrait for QuickSort {
    fn name(&self) -> &'static str {
        "quick_sort"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl<T: Ord> SortingAlgorithmTrait<T> for QuickSort {
    fn sort(&self, slice: &mut [T]) {
        Self::sort(slice);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut arr = [5, 2, 8, 1, 9, 4];
        QuickSort::sort(&mut arr);
        assert_eq!(arr, [1, 2, 4, 5, 8, 9]);
    }
}
