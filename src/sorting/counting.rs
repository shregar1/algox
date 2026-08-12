use crate::abstraction::AlgorithmTrait;
use super::abstraction::SortingAlgorithmTrait;

pub struct CountingSort;

impl CountingSort {
    pub fn sort(slice: &mut [usize]) {
        if slice.len() <= 1 {
            return;
        }

        let max = match slice.iter().max() {
            Some(&m) => m,
            None => return,
        };

        let mut count = vec![0usize; max + 1];
        for &num in slice.iter() {
            count[num] += 1;
        }

        let mut idx = 0;
        for (val, &c) in count.iter().enumerate() {
            for _ in 0..c {
                slice[idx] = val;
                idx += 1;
            }
        }
    }
}

impl AlgorithmTrait for CountingSort {
    fn name(&self) -> &'static str {
        "counting_sort"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl SortingAlgorithmTrait<usize> for CountingSort {
    fn sort(&self, slice: &mut [usize]) {
        Self::sort(slice);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_sort() {
        let mut arr = [4, 2, 2, 8, 3, 3, 1];
        CountingSort::sort(&mut arr);
        assert_eq!(arr, [1, 2, 2, 3, 3, 4, 8]);
    }
}
