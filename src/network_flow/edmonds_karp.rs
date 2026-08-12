use crate::abstraction::AlgorithmTrait;
use super::abstraction::NetworkFlowAlgorithmTrait;
use std::collections::VecDeque;

/// Edmonds-Karp algorithm (BFS-based Ford-Fulkerson) for maximum flow.
///
/// Build the graph using `new(nodes)` then `add_edge(u, v, cap)`.
/// Call `max_flow(source, sink)` to compute the result.
pub struct EdmondsKarp {
    n: usize,
    /// capacity[u][v]
    cap: Vec<Vec<i64>>,
}

impl EdmondsKarp {
    pub fn new(nodes: usize) -> Self {
        Self {
            n: nodes,
            cap: vec![vec![0; nodes]; nodes],
        }
    }

    /// Add a directed edge u → v with capacity `cap`.
    /// For undirected edges, call `add_edge(u,v,c)` and `add_edge(v,u,c)`.
    pub fn add_edge(&mut self, u: usize, v: usize, cap: i64) {
        self.cap[u][v] += cap;
    }

    fn bfs(&self, s: usize, t: usize, parent: &mut Vec<i64>) -> bool {
        parent.fill(-1);
        parent[s] = s as i64;
        let mut queue = VecDeque::new();
        queue.push_back(s);
        while let Some(u) = queue.pop_front() {
            for v in 0..self.n {
                if parent[v] == -1 && self.cap[u][v] > 0 {
                    parent[v] = u as i64;
                    if v == t { return true; }
                    queue.push_back(v);
                }
            }
        }
        false
    }
}

impl AlgorithmTrait for EdmondsKarp {
    fn name(&self) -> &'static str {
        "edmonds_karp"
    }

    fn len(&self) -> usize {
        self.n
    }

    fn clear(&mut self) {
        for row in &mut self.cap {
            row.fill(0);
        }
    }
}

impl NetworkFlowAlgorithmTrait for EdmondsKarp {
    fn max_flow(&mut self, s: usize, t: usize) -> i64 {
        let mut flow = 0i64;
        let mut parent = vec![-1i64; self.n];
        while self.bfs(s, t, &mut parent) {
            // find bottleneck
            let mut path_flow = i64::MAX;
            let mut v = t;
            while v != s {
                let u = parent[v] as usize;
                path_flow = path_flow.min(self.cap[u][v]);
                v = u;
            }
            // update capacities
            v = t;
            while v != s {
                let u = parent[v] as usize;
                self.cap[u][v] -= path_flow;
                self.cap[v][u] += path_flow;
                v = u;
            }
            flow += path_flow;
        }
        flow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edmonds_karp() {
        // Classic 6-node flow network; max flow s→t = 23
        let mut g = EdmondsKarp::new(6);
        g.add_edge(0, 1, 16);
        g.add_edge(0, 2, 13);
        g.add_edge(1, 2, 10);
        g.add_edge(1, 3, 12);
        g.add_edge(2, 1, 4);
        g.add_edge(2, 4, 14);
        g.add_edge(3, 2, 9);
        g.add_edge(3, 5, 20);
        g.add_edge(4, 3, 7);
        g.add_edge(4, 5, 4);
        assert_eq!(g.max_flow(0, 5), 23);
    }
}
