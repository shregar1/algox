use crate::abstraction::AlgorithmTrait;
use super::abstraction::SortingAlgorithmTrait;

pub struct SelectionSort;

impl SelectionSort {
    pub fn sort<T: Ord>(slice: &mut [T]) {
        let len = slice.len();
        if len <= 1 {
            return;
        }
        for i in 0..len {
            let mut min_idx = i;
            for j in i + 1..len {
                if slice[j] < slice[min_idx] {
                    min_idx = j;
                }
            }
            if min_idx != i {
                slice.swap(i, min_idx);
            }
        }
    }
}

impl AlgorithmTrait for SelectionSort {
    fn name(&self) -> &'static str {
        "selection_sort"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl<T: Ord> SortingAlgorithmTrait<T> for SelectionSort {
    fn sort(&self, slice: &mut [T]) {
        Self::sort(slice);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut arr = [5, 2, 8, 1, 9, 4];
        SelectionSort::sort(&mut arr);
        assert_eq!(arr, [1, 2, 4, 5, 8, 9]);
    }
}
