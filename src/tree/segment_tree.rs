use crate::abstraction::AlgorithmTrait;

/// Segment Tree for efficient Range Sum Queries and Point Updates in O(log n).
pub struct SegmentTree {
    n: usize,
    tree: Vec<i64>,
}

impl SegmentTree {
    /// Builds a SegmentTree from initial data slice.
    pub fn build(data: &[i64]) -> Self {
        let n = data.len();
        let tree = vec![0i64; 4 * n.max(1)];
        let mut st = Self { n, tree };
        if n > 0 {
            st.build_tree(data, 1, 0, n - 1);
        }
        st
    }

    fn build_tree(&mut self, data: &[i64], node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node] = data[start];
            return;
        }
        let mid = start + (end - start) / 2;
        self.build_tree(data, 2 * node, start, mid);
        self.build_tree(data, 2 * node + 1, mid + 1, end);
        self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
    }

    /// Update value at 0-indexed position `idx` to `val`.
    pub fn update(&mut self, idx: usize, val: i64) {
        if idx >= self.n {
            return;
        }
        self.update_tree(1, 0, self.n - 1, idx, val);
    }

    fn update_tree(&mut self, node: usize, start: usize, end: usize, idx: usize, val: i64) {
        if start == end {
            self.tree[node] = val;
            return;
        }
        let mid = start + (end - start) / 2;
        if idx <= mid {
            self.update_tree(2 * node, start, mid, idx, val);
        } else {
            self.update_tree(2 * node + 1, mid + 1, end, idx, val);
        }
        self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
    }

    /// Query range sum [l, r] inclusive (0-indexed).
    pub fn query_range(&self, l: usize, r: usize) -> i64 {
        if self.n == 0 || l > r || r >= self.n {
            return 0;
        }
        self.query_tree(1, 0, self.n - 1, l, r)
    }

    fn query_tree(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i64 {
        if r < start || end < l {
            return 0;
        }
        if l <= start && end <= r {
            return self.tree[node];
        }
        let mid = start + (end - start) / 2;
        let left_sum = self.query_tree(2 * node, start, mid, l, r);
        let right_sum = self.query_tree(2 * node + 1, mid + 1, end, l, r);
        left_sum + right_sum
    }
}

impl AlgorithmTrait for SegmentTree {
    fn name(&self) -> &'static str {
        "segment_tree"
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
    fn test_segment_tree_range_query() {
        let arr = [1, 3, 5, 7, 9, 11];
        let mut st = SegmentTree::build(&arr);
        assert_eq!(st.query_range(1, 3), 15); // 3 + 5 + 7
        assert_eq!(st.query_range(0, 5), 36);

        st.update(1, 10); // replace 3 with 10
        assert_eq!(st.query_range(1, 3), 22); // 10 + 5 + 7
    }
}
