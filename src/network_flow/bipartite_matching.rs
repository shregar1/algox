use crate::abstraction::AlgorithmTrait;
use super::abstraction::NetworkFlowAlgorithmTrait;

/// Bipartite Matching using augmenting paths (Hopcroft-Karp simplified).
///
/// Left vertices are `0..left`, right vertices are `0..right`.
/// Add edges with `add_edge(l, r)`, then call `max_matching()`.
pub struct BipartiteMatching {
    left: usize,
    right: usize,
    adj: Vec<Vec<usize>>,
    match_l: Vec<Option<usize>>,
    match_r: Vec<Option<usize>>,
}

impl BipartiteMatching {
    pub fn new(left: usize, right: usize) -> Self {
        Self {
            left,
            right,
            adj: vec![Vec::new(); left],
            match_l: vec![None; left],
            match_r: vec![None; right],
        }
    }

    pub fn add_edge(&mut self, l: usize, r: usize) {
        self.adj[l].push(r);
    }

    fn dfs_static(u: usize, adj: &[Vec<usize>], visited: &mut Vec<bool>, match_r: &mut Vec<Option<usize>>) -> bool {
        for &v in &adj[u] {
            if !visited[v] {
                visited[v] = true;
                let prev = match_r[v];
                if prev.is_none() || Self::dfs_static(prev.unwrap(), adj, visited, match_r) {
                    match_r[v] = Some(u);
                    return true;
                }
            }
        }
        false
    }

    /// Returns the size of the maximum bipartite matching.
    pub fn max_matching(&mut self) -> usize {
        self.match_l = vec![None; self.left];
        self.match_r = vec![None; self.right];
        let mut result = 0;
        for u in 0..self.left {
            let mut visited = vec![false; self.right];
            let mut match_r = std::mem::take(&mut self.match_r);
            let found = Self::dfs_static(u, &self.adj, &mut visited, &mut match_r);
            self.match_r = match_r;
            if found {
                result += 1;
            }
        }
        // rebuild match_l
        for (r, ml) in self.match_r.iter().enumerate() {
            if let Some(l) = ml {
                self.match_l[*l] = Some(r);
            }
        }
        result
    }

    /// Returns matched pairs (left, right) after `max_matching()`.
    pub fn matching(&self) -> Vec<(usize, usize)> {
        self.match_l.iter().enumerate()
            .filter_map(|(l, r)| r.map(|rv| (l, rv)))
            .collect()
    }
}

impl AlgorithmTrait for BipartiteMatching {
    fn name(&self) -> &'static str {
        "bipartite_matching"
    }

    fn len(&self) -> usize {
        self.left + self.right
    }

    fn clear(&mut self) {
        for row in &mut self.adj { row.clear(); }
        self.match_l.fill(None);
        self.match_r.fill(None);
    }
}

impl NetworkFlowAlgorithmTrait for BipartiteMatching {
    fn max_flow(&mut self, _s: usize, _t: usize) -> i64 {
        self.max_matching() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bipartite_matching() {
        let mut bm = BipartiteMatching::new(3, 3);
        bm.add_edge(0, 0);
        bm.add_edge(0, 1);
        bm.add_edge(1, 1);
        bm.add_edge(2, 2);
        assert_eq!(bm.max_matching(), 3);
    }
}
