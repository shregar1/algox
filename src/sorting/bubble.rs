use crate::abstraction::AlgorithmTrait;
use super::abstraction::SortingAlgorithmTrait;

pub struct BubbleSort;

impl BubbleSort {
    pub fn sort<T: Ord>(slice: &mut [T]) {
        let len = slice.len();
        if len <= 1 {
            return;
        }
        for i in 0..len {
            let mut swapped = false;
            for j in 0..len - 1 - i {
                if slice[j] > slice[j + 1] {
                    slice.swap(j, j + 1);
                    swapped = true;
                }
            }
            if !swapped {
                break;
            }
        }
    }
}

impl AlgorithmTrait for BubbleSort {
    fn name(&self) -> &'static str {
        "bubble_sort"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl<T: Ord> SortingAlgorithmTrait<T> for BubbleSort {
    fn sort(&self, slice: &mut [T]) {
        Self::sort(slice);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = [5, 2, 8, 1, 9, 4];
        BubbleSort::sort(&mut arr);
        assert_eq!(arr, [1, 2, 4, 5, 8, 9]);
    }
}
