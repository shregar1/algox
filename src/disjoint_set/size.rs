use crate::abstraction::AlgorithmTrait;
use super::abstraction::DisjointSetAlgorithmTrait;

pub struct DisjointSetSize {
    parent: Vec<usize>,
    size: Vec<usize>,
    sets_count: usize,
}

impl DisjointSetSize {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            sets_count: n,
        }
    }

    pub fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            let p = self.parent[i];
            self.parent[i] = self.find(p);
            self.parent[i]
        }
    }

    pub fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
            self.sets_count -= 1;
            true
        } else {
            false
        }
    }

    pub fn is_same_set(&mut self, i: usize, j: usize) -> bool {
        self.find(i) == self.find(j)
    }

    pub fn set_size(&mut self, i: usize) -> usize {
        let root = self.find(i);
        self.size[root]
    }

    pub fn sets_count(&self) -> usize {
        self.sets_count
    }
}

impl AlgorithmTrait for DisjointSetSize {
    fn name(&self) -> &'static str {
        "disjoint_set_size"
    }

    fn len(&self) -> usize {
        self.sets_count
    }

    fn clear(&mut self) {
        let n = self.parent.len();
        self.parent = (0..n).collect();
        self.size.fill(1);
        self.sets_count = n;
    }
}

impl DisjointSetAlgorithmTrait for DisjointSetSize {
    fn find(&mut self, i: usize) -> usize {
        self.find(i)
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        self.union(i, j)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disjoint_set_size() {
        let mut dsu = DisjointSetSize::new(5);
        assert!(dsu.union(0, 1));
        assert!(dsu.union(1, 2));
        assert_eq!(dsu.set_size(0), 3);
        assert_eq!(dsu.sets_count(), 3);
    }
}
