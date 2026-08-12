use crate::abstraction::AlgorithmTrait;
use super::abstraction::BufferAlgorithmTrait;

pub struct RingBuffer<T> {
    data: Vec<Option<T>>,
    head: usize,
    tail: usize,
    len: usize,
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let cap = capacity.max(1);
        Self {
            data: (0..cap).map(|_| None).collect(),
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    pub fn with_default(capacity: usize) -> Self
    where
        T: Default + Clone,
    {
        let cap = capacity.max(1);
        Self {
            data: (0..cap).map(|_| Some(T::default())).collect(),
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.data.len()
    }

    pub fn push_back(&mut self, value: T) -> Result<(), T> {
        if self.is_full() {
            return Err(value);
        }
        self.data[self.tail] = Some(value);
        self.tail = (self.tail + 1) % self.data.len();
        self.len += 1;
        Ok(())
    }

    pub fn push_front(&mut self, value: T) -> Result<(), T> {
        if self.is_full() {
            return Err(value);
        }
        self.head = if self.head == 0 {
            self.data.len() - 1
        } else {
            self.head - 1
        };
        self.data[self.head] = Some(value);
        self.len += 1;
        Ok(())
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.tail = if self.tail == 0 {
            self.data.len() - 1
        } else {
            self.tail - 1
        };
        let v = self.data[self.tail].take();
        self.len -= 1;
        v
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let v = self.data[self.head].take();
        self.head = (self.head + 1) % self.data.len();
        self.len -= 1;
        v
    }

    pub fn front(&self) -> Option<&T> {
        self.data[self.head].as_ref()
    }

    pub fn back(&self) -> Option<&T> {
        let idx = if self.tail == 0 {
            self.data.len() - 1
        } else {
            self.tail - 1
        };
        self.data[idx].as_ref()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        let idx = (self.head + index) % self.data.len();
        self.data[idx].as_ref()
    }

    pub fn clear(&mut self) {
        for slot in self.data.iter_mut() {
            *slot = None;
        }
        self.head = 0;
        self.tail = 0;
        self.len = 0;
    }

    pub fn iter(&self) -> RingBufferIter<'_, T> {
        RingBufferIter {
            buf: self,
            index: 0,
        }
    }
}

impl<T> AlgorithmTrait for RingBuffer<T> {
    fn name(&self) -> &'static str {
        "ring_buffer"
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

impl<T> BufferAlgorithmTrait<T> for RingBuffer<T> {
    fn push_back(&mut self, value: T) -> Result<(), T> {
        self.push_back(value)
    }

    fn pop_front(&mut self) -> Option<T> {
        self.pop_front()
    }

    fn front(&self) -> Option<&T> {
        self.front()
    }

    fn back(&self) -> Option<&T> {
        self.back()
    }

    fn capacity(&self) -> usize {
        self.capacity()
    }
}

pub struct RingBufferIter<'a, T> {
    buf: &'a RingBuffer<T>,
    index: usize,
}

impl<'a, T> Iterator for RingBufferIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.buf.get(self.index);
        if item.is_some() {
            self.index += 1;
        }
        item
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_empty() {
        let rb: RingBuffer<i32> = RingBuffer::new(4);
        assert_eq!(rb.capacity(), 4);
        assert_eq!(rb.len(), 0);
        assert!(rb.is_empty());
    }

    #[test]
    fn test_push_back_pop_front() {
        let mut rb = RingBuffer::new(4);
        rb.push_back(1).unwrap();
        rb.push_back(2).unwrap();
        rb.push_back(3).unwrap();
        assert_eq!(rb.pop_front(), Some(1));
        assert_eq!(rb.pop_front(), Some(2));
        assert_eq!(rb.len(), 1);
    }

    #[test]
    fn test_push_back_full() {
        let mut rb = RingBuffer::new(2);
        rb.push_back(1).unwrap();
        rb.push_back(2).unwrap();
        assert_eq!(rb.push_back(3), Err(3));
        assert!(rb.is_full());
    }

    #[test]
    fn test_push_front() {
        let mut rb = RingBuffer::new(3);
        rb.push_front(1).unwrap();
        rb.push_front(2).unwrap();
        assert_eq!(rb.front(), Some(&2));
        assert_eq!(rb.back(), Some(&1));
    }

    #[test]
    fn test_pop_back() {
        let mut rb = RingBuffer::new(3);
        rb.push_back(1).unwrap();
        rb.push_back(2).unwrap();
        assert_eq!(rb.pop_back(), Some(2));
        assert_eq!(rb.pop_back(), Some(1));
        assert_eq!(rb.pop_back(), None);
    }

    #[test]
    fn test_wraparound() {
        let mut rb = RingBuffer::new(3);
        rb.push_back(1).unwrap();
        rb.push_back(2).unwrap();
        rb.push_back(3).unwrap();
        rb.pop_front();
        rb.push_back(4).unwrap();
        let collected: Vec<i32> = rb.iter().copied().collect();
        assert_eq!(collected, vec![2, 3, 4]);
    }

    #[test]
    fn test_get() {
        let mut rb = RingBuffer::new(4);
        rb.push_back(10).unwrap();
        rb.push_back(20).unwrap();
        rb.push_back(30).unwrap();
        assert_eq!(rb.get(0), Some(&10));
        assert_eq!(rb.get(1), Some(&20));
        assert_eq!(rb.get(2), Some(&30));
        assert_eq!(rb.get(3), None);
    }

    #[test]
    fn test_clear() {
        let mut rb = RingBuffer::new(3);
        rb.push_back(1).unwrap();
        rb.push_back(2).unwrap();
        rb.clear();
        assert!(rb.is_empty());
        assert_eq!(rb.front(), None);
    }

    #[test]
    fn test_iter() {
        let mut rb = RingBuffer::new(4);
        rb.push_back(1).unwrap();
        rb.push_back(2).unwrap();
        rb.push_back(3).unwrap();
        let s: Vec<i32> = rb.iter().copied().collect();
        assert_eq!(s, vec![1, 2, 3]);
    }
}
