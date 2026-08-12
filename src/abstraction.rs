//! Common algorithm abstraction trait for `rivex-algorithm`.

/// Common trait for algorithm data structures and utilities.
pub trait AlgorithmTrait {
    /// Returns the name identifier of the algorithm.
    fn name(&self) -> &'static str;

    /// Returns the current number of elements/items stored or tracked by the algorithm.
    fn len(&self) -> usize;

    /// Returns `true` if the algorithm data structure is currently empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears or resets the state of the algorithm data structure.
    fn clear(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockAlgo {
        count: usize,
    }

    impl AlgorithmTrait for MockAlgo {
        fn name(&self) -> &'static str {
            "mock_algo"
        }
        fn len(&self) -> usize {
            self.count
        }
        fn clear(&mut self) {
            self.count = 0;
        }
    }

    #[test]
    fn test_algorithm_trait_defaults() {
        let mut mock = MockAlgo { count: 2 };
        assert_eq!(mock.name(), "mock_algo");
        assert_eq!(mock.len(), 2);
        assert!(!mock.is_empty());
        mock.clear();
        assert_eq!(mock.len(), 0);
        assert!(mock.is_empty());
    }
}
