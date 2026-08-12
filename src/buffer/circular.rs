use crate::abstraction::AlgorithmTrait;
use super::abstraction::BufferAlgorithmTrait;

/// Fixed-capacity Circular (Ring) Buffer that automatically overwrites oldest items when full.
pub struct CircularBuffer<T> {
    data: Vec<Option<T>>,
    write_idx: usize,
    read_idx: usize,
    size: usize,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let cap = capacity.max(1);
        Self {
            data: (0..cap).map(|_| None).collect(),
            write_idx: 0,
            read_idx: 0,
            size: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.data.len()
    }

    /// Push element, overwriting oldest element if full. Returns overwritten element if any.
    pub fn push(&mut self, value: T) -> Option<T> {
        let old = self.data[self.write_idx].take();
        self.data[self.write_idx] = Some(value);
        self.write_idx = (self.write_idx + 1) % self.data.len();

        if self.size < self.data.len() {
            self.size += 1;
            None
        } else {
            self.read_idx = (self.read_idx + 1) % self.data.len();
            old
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        let val = self.data[self.read_idx].take();
        self.read_idx = (self.read_idx + 1) % self.data.len();
        self.size -= 1;
        val
    }

    pub fn peek(&self) -> Option<&T> {
        if self.size == 0 {
            None
        } else {
            self.data[self.read_idx].as_ref()
        }
    }

    pub fn clear(&mut self) {
        for slot in self.data.iter_mut() {
            *slot = None;
        }
        self.write_idx = 0;
        self.read_idx = 0;
        self.size = 0;
    }
}

impl<T> AlgorithmTrait for CircularBuffer<T> {
    fn name(&self) -> &'static str {
        "circular_buffer"
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

impl<T> BufferAlgorithmTrait<T> for CircularBuffer<T> {
    fn push_back(&mut self, value: T) -> Result<(), T> {
        self.push(value);
        Ok(())
    }

    fn pop_front(&mut self) -> Option<T> {
        self.pop()
    }

    fn front(&self) -> Option<&T> {
        self.peek()
    }

    fn back(&self) -> Option<&T> {
        if self.size == 0 {
            None
        } else {
            let last_idx = if self.write_idx == 0 {
                self.data.len() - 1
            } else {
                self.write_idx - 1
            };
            self.data[last_idx].as_ref()
        }
    }

    fn capacity(&self) -> usize {
        self.capacity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_buffer_overwrite() {
        let mut cb = CircularBuffer::new(2);
        assert_eq!(cb.name(), "circular_buffer");
        assert_eq!(cb.capacity(), 2);
        assert!(cb.is_empty());
        assert!(!cb.is_full());

        assert_eq!(cb.push(1), None);
        assert_eq!(cb.push(2), None);
        assert!(cb.is_full());
        assert_eq!(cb.front(), Some(&1));
        assert_eq!(cb.back(), Some(&2));

        assert_eq!(cb.push(3), Some(1)); // Overwrites 1
        assert_eq!(cb.front(), Some(&2));
        assert_eq!(cb.back(), Some(&3));

        assert_eq!(cb.pop_front(), Some(2));
        assert_eq!(cb.pop(), Some(3));
        assert_eq!(cb.pop(), None);
        assert!(cb.is_empty());
    }

    #[test]
    fn test_circular_buffer_traits_and_clear() {
        let mut cb = CircularBuffer::new(3);
        let _ = cb.push_back(10);
        let _ = cb.push_back(20);
        assert_eq!(cb.len(), 2);
        cb.clear();
        assert_eq!(cb.len(), 0);
        assert!(cb.is_empty());
        assert_eq!(cb.front(), None);
        assert_eq!(cb.back(), None);
    }
}
