use crate::abstraction::AlgorithmTrait;
use super::abstraction::GreedyAlgorithmTrait;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
enum HuffmanNode {
    Leaf { ch: char, freq: usize },
    Internal { freq: usize, left: Box<HuffmanNode>, right: Box<HuffmanNode> },
}

impl HuffmanNode {
    fn freq(&self) -> usize {
        match self {
            HuffmanNode::Leaf { freq, .. } => *freq,
            HuffmanNode::Internal { freq, .. } => *freq,
        }
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq().cmp(&self.freq())
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Huffman Coding algorithm for optimal prefix-free codes.
pub struct HuffmanCoding;

impl HuffmanCoding {
    /// Builds Huffman prefix codes for each character in `text`.
    pub fn build_codes(text: &str) -> HashMap<char, String> {
        if text.is_empty() {
            return HashMap::new();
        }

        let mut freq_map = HashMap::new();
        for ch in text.chars() {
            *freq_map.entry(ch).or_insert(0) += 1;
        }

        let mut heap = BinaryHeap::new();
        for (ch, freq) in freq_map {
            heap.push(HuffmanNode::Leaf { ch, freq });
        }

        if heap.len() == 1 {
            let mut codes = HashMap::new();
            if let Some(HuffmanNode::Leaf { ch, .. }) = heap.pop() {
                codes.insert(ch, "0".to_string());
            }
            return codes;
        }

        while heap.len() > 1 {
            let left = heap.pop().unwrap();
            let right = heap.pop().unwrap();
            let parent = HuffmanNode::Internal {
                freq: left.freq() + right.freq(),
                left: Box::new(left),
                right: Box::new(right),
            };
            heap.push(parent);
        }

        let mut codes = HashMap::new();
        if let Some(root) = heap.pop() {
            Self::generate_codes(&root, String::new(), &mut codes);
        }
        codes
    }

    fn generate_codes(node: &HuffmanNode, prefix: String, codes: &mut HashMap<char, String>) {
        match node {
            HuffmanNode::Leaf { ch, .. } => {
                codes.insert(*ch, prefix);
            }
            HuffmanNode::Internal { left, right, .. } => {
                Self::generate_codes(left, format!("{}0", prefix), codes);
                Self::generate_codes(right, format!("{}1", prefix), codes);
            }
        }
    }
}

impl AlgorithmTrait for HuffmanCoding {
    fn name(&self) -> &'static str {
        "huffman_coding"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl GreedyAlgorithmTrait for HuffmanCoding {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_huffman_coding() {
        let codes = HuffmanCoding::build_codes("abracadabra");
        assert!(codes.contains_key(&'a'));
        assert!(codes.contains_key(&'b'));
        assert!(codes.get(&'a').unwrap().len() < codes.get(&'c').unwrap().len());
    }
}
