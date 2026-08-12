use crate::abstraction::AlgorithmTrait;

/// Fenwick Tree (Binary Indexed Tree) for O(log n) prefix sums and point updates.
pub struct FenwickTree {
    n: usize,
    tree: Vec<i64>,
}

impl FenwickTree {
    /// Creates a new Fenwick Tree with specified capacity `n`.
    pub fn new(n: usize) -> Self {
        Self {
            n,
            tree: vec![0i64; n + 1],
        }
    }

    /// Builds a Fenwick Tree from an initial array slice.
    pub fn build(data: &[i64]) -> Self {
        let n = data.len();
        let mut ft = Self::new(n);
        for (i, &val) in data.iter().enumerate() {
            ft.add(i, val);
        }
        ft
    }

    /// Add `val` to 0-indexed position `idx`.
    pub fn add(&mut self, idx: usize, val: i64) {
        let mut i = (idx + 1) as i64;
        let n = self.n as i64;
        while i <= n {
            self.tree[i as usize] += val;
            i += i & (-i);
        }
    }

    /// Returns prefix sum from 0 to `idx` inclusive (0-indexed).
    pub fn prefix_sum(&self, idx: usize) -> i64 {
        if idx >= self.n {
            return 0;
        }
        let mut sum = 0i64;
        let mut i = (idx + 1) as i64;
        while i > 0 {
            sum += self.tree[i as usize];
            i -= i & (-i);
        }
        sum
    }

    /// Query range sum [l, r] inclusive (0-indexed).
    pub fn query_range(&self, l: usize, r: usize) -> i64 {
        if l > r || r >= self.n {
            return 0;
        }
        if l == 0 {
            self.prefix_sum(r)
        } else {
            self.prefix_sum(r) - self.prefix_sum(l - 1)
        }
    }
}

impl AlgorithmTrait for FenwickTree {
    fn name(&self) -> &'static str {
        "fenwick_tree"
    }

    fn len(&self) -> usize {
        self.n
    }

    fn clear(&mut self) {
        self.n = 0;
        self.tree.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fenwick_tree() {
        let arr = [2, 1, 1, 3, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut ft = FenwickTree::build(&arr);
        assert_eq!(ft.prefix_sum(5), 12);
        assert_eq!(ft.query_range(1, 5), 10);

        ft.add(3, 6); // Add 6 to index 3
        assert_eq!(ft.prefix_sum(5), 18);
    }
}
