use crate::abstraction::AlgorithmTrait;
use super::abstraction::HeapAlgorithmTrait;
use std::collections::BinaryHeap;

pub struct BinaryMinHeap<T> {
    heap: BinaryHeap<std::cmp::Reverse<T>>,
}

impl<T: Ord> BinaryMinHeap<T> {
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
        self.heap.push(std::cmp::Reverse(item));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|r| r.0)
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|r| &r.0)
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

impl<T: Ord> Default for BinaryMinHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> AlgorithmTrait for BinaryMinHeap<T> {
    fn name(&self) -> &'static str {
        "min"
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

impl<T: Ord> HeapAlgorithmTrait<T> for BinaryMinHeap<T> {
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
    fn test_min_heap() {
        let mut heap = BinaryMinHeap::new();
        heap.push(5);
        heap.push(1);
        heap.push(3);
        assert_eq!(heap.peek(), Some(&1));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(5));
    }
}
