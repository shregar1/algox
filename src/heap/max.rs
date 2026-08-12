use crate::abstraction::AlgorithmTrait;
use super::abstraction::HeapAlgorithmTrait;
use std::collections::BinaryHeap;

pub struct BinaryMaxHeap<T> {
    heap: BinaryHeap<T>,
}

impl<T: Ord> BinaryMaxHeap<T> {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: BinaryHeap::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, item: T) {
        self.heap.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.peek()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn clear(&mut self) {
        self.heap.clear();
    }
}

impl<T: Ord> Default for BinaryMaxHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> AlgorithmTrait for BinaryMaxHeap<T> {
    fn name(&self) -> &'static str {
        "max"
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn clear(&mut self) {
        self.clear();
    }
}

impl<T: Ord> HeapAlgorithmTrait<T> for BinaryMaxHeap<T> {
    fn push(&mut self, item: T) {
        self.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.peek()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_heap() {
        let mut heap = BinaryMaxHeap::new();
        heap.push(1);
        heap.push(5);
        heap.push(3);
        assert_eq!(heap.peek(), Some(&5));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(1));
    }
}
