use crate::abstraction::AlgorithmTrait;
use super::abstraction::GraphAlgorithmTrait;

/// Tarjan's algorithm for finding Strongly Connected Components (SCC) in directed graphs.
pub struct TarjanSCC;

impl TarjanSCC {
    /// Returns a vector of SCCs, where each SCC is a vector of node indices.
    pub fn find_sccs(nodes: usize, adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
        let mut index = 0;
        let mut indices = vec![usize::MAX; nodes];
        let mut lowlink = vec![usize::MAX; nodes];
        let mut on_stack = vec![false; nodes];
        let mut stack = Vec::new();
        let mut sccs = Vec::new();

        for u in 0..nodes {
            if indices[u] == usize::MAX {
                Self::strongconnect(
                    u,
                    adj,
                    &mut index,
                    &mut indices,
                    &mut lowlink,
                    &mut on_stack,
                    &mut stack,
                    &mut sccs,
                );
            }
        }
        sccs
    }

    fn strongconnect(
        u: usize,
        adj: &[Vec<usize>],
        index: &mut usize,
        indices: &mut [usize],
        lowlink: &mut [usize],
        on_stack: &mut [bool],
        stack: &mut Vec<usize>,
        sccs: &mut Vec<Vec<usize>>,
    ) {
        indices[u] = *index;
        lowlink[u] = *index;
        *index += 1;
        stack.push(u);
        on_stack[u] = true;

        for &v in &adj[u] {
            if indices[v] == usize::MAX {
                Self::strongconnect(v, adj, index, indices, lowlink, on_stack, stack, sccs);
                lowlink[u] = lowlink[u].min(lowlink[v]);
            } else if on_stack[v] {
                lowlink[u] = lowlink[u].min(indices[v]);
            }
        }

        if lowlink[u] == indices[u] {
            let mut scc = Vec::new();
            loop {
                let v = stack.pop().unwrap();
                on_stack[v] = false;
                scc.push(v);
                if v == u {
                    break;
                }
            }
            sccs.push(scc);
        }
    }
}

impl AlgorithmTrait for TarjanSCC {
    fn name(&self) -> &'static str {
        "tarjan_scc"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl GraphAlgorithmTrait for TarjanSCC {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tarjan_scc() {
        let adj = vec![
            vec![1],       // 0 -> 1
            vec![2],       // 1 -> 2
            vec![0, 3],    // 2 -> 0, 3 (0,1,2 form SCC)
            vec![4],       // 3 -> 4
            vec![],        // 4
        ];

        let sccs = TarjanSCC::find_sccs(5, &adj);
        assert_eq!(sccs.len(), 3); // {4}, {3}, {0,1,2}
    }
}
