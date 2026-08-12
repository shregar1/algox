use crate::abstraction::AlgorithmTrait;
use super::abstraction::SortingAlgorithmTrait;

pub struct RadixSort;

impl RadixSort {
    pub fn sort(slice: &mut [u64]) {
        if slice.len() <= 1 {
            return;
        }

        let max = match slice.iter().max() {
            Some(&m) => m,
            None => return,
        };

        let mut exp = 1u64;
        let mut buffer = vec![0u64; slice.len()];

        while max / exp > 0 {
            let mut count = [0usize; 10];

            for &num in slice.iter() {
                let digit = ((num / exp) % 10) as usize;
                count[digit] += 1;
            }

            for i in 1..10 {
                count[i] += count[i - 1];
            }

            for &num in slice.iter().rev() {
                let digit = ((num / exp) % 10) as usize;
                count[digit] -= 1;
                buffer[count[digit]] = num;
            }

            slice.copy_from_slice(&buffer);

            if exp.checked_mul(10).is_none() {
                break;
            }
            exp *= 10;
        }
    }
}

impl AlgorithmTrait for RadixSort {
    fn name(&self) -> &'static str {
        "radix_sort"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl SortingAlgorithmTrait<u64> for RadixSort {
    fn sort(&self, slice: &mut [u64]) {
        Self::sort(slice);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix_sort() {
        let mut arr = [170, 45, 75, 90, 802, 24, 2, 66];
        RadixSort::sort(&mut arr);
        assert_eq!(arr, [2, 24, 45, 66, 75, 90, 170, 802]);
    }
}
