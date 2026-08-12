use crate::abstraction::AlgorithmTrait;
use super::abstraction::SortingAlgorithmTrait;

pub struct HeapSort;

impl HeapSort {
    pub fn sort<T: Ord>(slice: &mut [T]) {
        let len = slice.len();
        if len <= 1 {
            return;
        }

        for i in (0..len / 2).rev() {
            Self::heapify(slice, len, i);
        }

        for i in (1..len).rev() {
            slice.swap(0, i);
            Self::heapify(slice, i, 0);
        }
    }

    fn heapify<T: Ord>(slice: &mut [T], n: usize, i: usize) {
        let mut largest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;

        if left < n && slice[left] > slice[largest] {
            largest = left;
        }

        if right < n && slice[right] > slice[largest] {
            largest = right;
        }

        if largest != i {
            slice.swap(i, largest);
            Self::heapify(slice, n, largest);
        }
    }
}

impl AlgorithmTrait for HeapSort {
    fn name(&self) -> &'static str {
        "heap_sort"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl<T: Ord> SortingAlgorithmTrait<T> for HeapSort {
    fn sort(&self, slice: &mut [T]) {
        Self::sort(slice);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_sort() {
        let mut arr = [5, 2, 8, 1, 9, 4];
        HeapSort::sort(&mut arr);
        assert_eq!(arr, [1, 2, 4, 5, 8, 9]);
    }
}
