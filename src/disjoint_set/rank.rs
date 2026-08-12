use crate::abstraction::AlgorithmTrait;
use super::abstraction::DisjointSetAlgorithmTrait;

pub struct DisjointSetRank {
    parent: Vec<usize>,
    rank: Vec<usize>,
    sets_count: usize,
}

impl DisjointSetRank {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
            sets_count: size,
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
            if self.rank[root_i] < self.rank[root_j] {
                self.parent[root_i] = root_j;
            } else if self.rank[root_i] > self.rank[root_j] {
                self.parent[root_j] = root_i;
            } else {
                self.parent[root_j] = root_i;
                self.rank[root_i] += 1;
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

    pub fn sets_count(&self) -> usize {
        self.sets_count
    }
}

impl AlgorithmTrait for DisjointSetRank {
    fn name(&self) -> &'static str {
        "disjoint_set_rank"
    }

    fn len(&self) -> usize {
        self.sets_count
    }

    fn clear(&mut self) {
        let size = self.parent.len();
        self.parent = (0..size).collect();
        self.rank.fill(0);
        self.sets_count = size;
    }
}

impl DisjointSetAlgorithmTrait for DisjointSetRank {
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
    fn test_disjoint_set_rank() {
        let mut dsu = DisjointSetRank::new(5);
        assert!(dsu.union(0, 1));
        assert!(dsu.union(1, 2));
        assert!(dsu.is_same_set(0, 2));
        assert!(!dsu.is_same_set(0, 3));
    }
}
