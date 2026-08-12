use crate::abstraction::AlgorithmTrait;
use super::abstraction::StringAlgorithmTrait;

pub struct AhoCorasickNode {
    pub children: std::collections::HashMap<u8, usize>,
    pub fail: usize,
    pub output: Vec<usize>,
}

pub struct AhoCorasick {
    nodes: Vec<AhoCorasickNode>,
}

impl AhoCorasick {
    pub fn new(patterns: &[&[u8]]) -> Self {
        let mut ac = Self {
            nodes: vec![AhoCorasickNode {
                children: std::collections::HashMap::new(),
                fail: 0,
                output: Vec::new(),
            }],
        };

        for (idx, pat) in patterns.iter().enumerate() {
            let mut curr = 0;
            for &byte in *pat {
                curr = if let Some(&next) = ac.nodes[curr].children.get(&byte) {
                    next
                } else {
                    let next = ac.nodes.len();
                    ac.nodes.push(AhoCorasickNode {
                        children: std::collections::HashMap::new(),
                        fail: 0,
                        output: Vec::new(),
                    });
                    ac.nodes[curr].children.insert(byte, next);
                    next
                };
            }
            ac.nodes[curr].output.push(idx);
        }

        ac.build_failure_links();
        ac
    }

    fn build_failure_links(&mut self) {
        let mut queue = std::collections::VecDeque::new();
        let root_children: Vec<(u8, usize)> = self.nodes[0]
            .children
            .iter()
            .map(|(&b, &n)| (b, n))
            .collect();

        for (_, child) in root_children {
            self.nodes[child].fail = 0;
            queue.push_back(child);
        }

        while let Some(curr) = queue.pop_front() {
            let children: Vec<(u8, usize)> = self.nodes[curr]
                .children
                .iter()
                .map(|(&b, &n)| (b, n))
                .collect();

            for (byte, child) in children {
                let mut fail = self.nodes[curr].fail;
                while fail != 0 && !self.nodes[fail].children.contains_key(&byte) {
                    fail = self.nodes[fail].fail;
                }
                if let Some(&next_fail) = self.nodes[fail].children.get(&byte) {
                    self.nodes[child].fail = next_fail;
                    let add_output = self.nodes[next_fail].output.clone();
                    self.nodes[child].output.extend(add_output);
                } else {
                    self.nodes[child].fail = 0;
                }
                queue.push_back(child);
            }
        }
    }

    pub fn find_all(&self, text: &[u8]) -> Vec<(usize, usize)> {
        let mut results = Vec::new();
        let mut curr = 0;

        for (i, &byte) in text.iter().enumerate() {
            while curr != 0 && !self.nodes[curr].children.contains_key(&byte) {
                curr = self.nodes[curr].fail;
            }
            if let Some(&next) = self.nodes[curr].children.get(&byte) {
                curr = next;
            }
            for &pat_idx in &self.nodes[curr].output {
                results.push((i, pat_idx));
            }
        }

        results
    }
}

impl AlgorithmTrait for AhoCorasick {
    fn name(&self) -> &'static str {
        "aho_corasick"
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn clear(&mut self) {
        self.nodes.truncate(1);
        self.nodes[0].children.clear();
        self.nodes[0].output.clear();
    }
}

impl StringAlgorithmTrait for AhoCorasick {
    fn compute(&self, text: &str, _pattern: &str) -> usize {
        self.find_all(text.as_bytes()).len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aho_corasick() {
        let patterns = vec![b"he".as_slice(), b"she".as_slice(), b"his".as_slice(), b"hers".as_slice()];
        let ac = AhoCorasick::new(&patterns);
        let matches = ac.find_all(b"ushers");
        assert!(!matches.is_empty());
    }
}
