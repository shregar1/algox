use crate::abstraction::AlgorithmTrait;
use super::abstraction::SortingAlgorithmTrait;

pub struct MergeSort;

impl MergeSort {
    pub fn sort<T: Ord + Clone>(slice: &mut [T]) {
        let len = slice.len();
        if len <= 1 {
            return;
        }

        let mid = len / 2;
        let mut left = slice[..mid].to_vec();
        let mut right = slice[mid..].to_vec();

        Self::sort(&mut left);
        Self::sort(&mut right);

        let mut i = 0;
        let mut j = 0;
        let mut k = 0;

        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                slice[k] = left[i].clone();
                i += 1;
            } else {
                slice[k] = right[j].clone();
                j += 1;
            }
            k += 1;
        }

        while i < left.len() {
            slice[k] = left[i].clone();
            i += 1;
            k += 1;
        }

        while j < right.len() {
            slice[k] = right[j].clone();
            j += 1;
            k += 1;
        }
    }
}

impl AlgorithmTrait for MergeSort {
    fn name(&self) -> &'static str {
        "merge_sort"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl<T: Ord + Clone> SortingAlgorithmTrait<T> for MergeSort {
    fn sort(&self, slice: &mut [T]) {
        Self::sort(slice);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut arr = [5, 2, 8, 1, 9, 4];
        MergeSort::sort(&mut arr);
        assert_eq!(arr, [1, 2, 4, 5, 8, 9]);
    }
}
