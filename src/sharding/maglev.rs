use crate::abstraction::AlgorithmTrait;
use super::abstraction::ShardingAlgorithmTrait;
use crate::hashing::fnv1a_64;

/// Google's Maglev Consistent Hashing implementation.
pub struct MaglevSharding {
    m: usize, // Table size (must be prime)
    lookup: Vec<Option<String>>,
    nodes: Vec<String>,
}

impl MaglevSharding {
    /// Creates a new Maglev Hash table of size `m` (prime number, e.g. 65537).
    pub fn new(m: usize) -> Self {
        let m = if m == 0 { 65537 } else { m };
        Self {
            m,
            lookup: vec![None; m],
            nodes: Vec::new(),
        }
    }

    /// Sets nodes and rebuilds the Maglev lookup table.
    pub fn set_nodes(&mut self, nodes: Vec<String>) {
        self.nodes = nodes;
        self.rebuild_lookup();
    }

    fn rebuild_lookup(&mut self) {
        if self.nodes.is_empty() {
            self.lookup.fill(None);
            return;
        }

        let n = self.nodes.len();
        let mut permutations = Vec::with_capacity(n);

        for node in &self.nodes {
            let h1 = fnv1a_64(format!("{}-h1", node).as_bytes()) as usize;
            let h2 = fnv1a_64(format!("{}-h2", node).as_bytes()) as usize;

            let offset = h1 % self.m;
            let skip = (h2 % (self.m - 1)) + 1;

            let mut perm = Vec::with_capacity(self.m);
            for j in 0..self.m {
                perm.push((offset + j * skip) % self.m);
            }
            permutations.push(perm);
        }

        let mut next = vec![0; n];
        let mut entry = vec![None; self.m];
        let mut count = 0;

        loop {
            for i in 0..n {
                let mut c = permutations[i][next[i]];
                while entry[c].is_some() {
                    next[i] += 1;
                    c = permutations[i][next[i]];
                }
                entry[c] = Some(self.nodes[i].clone());
                next[i] += 1;
                count += 1;
                if count == self.m {
                    self.lookup = entry;
                    return;
                }
            }
        }
    }

    /// O(1) node lookup for a key.
    pub fn get_node(&self, key: &str) -> Option<String> {
        if self.lookup.is_empty() {
            return None;
        }
        let hash = fnv1a_64(key.as_bytes()) as usize;
        let idx = hash % self.m;
        self.lookup[idx].clone()
    }
}

impl AlgorithmTrait for MaglevSharding {
    fn name(&self) -> &'static str {
        "maglev_sharding"
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn clear(&mut self) {
        self.nodes.clear();
        self.lookup.fill(None);
    }
}

impl ShardingAlgorithmTrait for MaglevSharding {
    fn get_shard(&self, key: &str) -> Option<String> {
        self.get_node(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maglev_sharding() {
        let mut mag = MaglevSharding::new(97);
        mag.set_nodes(vec!["node-1".into(), "node-2".into(), "node-3".into()]);

        let target1 = mag.get_node("request_abc").unwrap();
        let target2 = mag.get_node("request_abc").unwrap();
        assert_eq!(target1, target2);
        assert!(["node-1", "node-2", "node-3"].contains(&target1.as_str()));
    }
}
