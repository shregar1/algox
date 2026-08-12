use crate::abstraction::AlgorithmTrait;
use super::abstraction::TreeAlgorithmTrait;
use std::collections::HashMap;

struct TrieNode<V> {
    children: HashMap<char, TrieNode<V>>,
    value: Option<V>,
}

impl<V> Default for TrieNode<V> {
    fn default() -> Self {
        Self {
            children: HashMap::new(),
            value: None,
        }
    }
}

pub struct Trie<V> {
    root: TrieNode<V>,
    size: usize,
}

impl<V> Trie<V> {
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
            size: 0,
        }
    }

    pub fn insert(&mut self, key: &str, value: V) -> Option<V> {
        let mut curr = &mut self.root;
        for ch in key.chars() {
            curr = curr.children.entry(ch).or_default();
        }
        let old = curr.value.take();
        curr.value = Some(value);
        if old.is_none() {
            self.size += 1;
        }
        old
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        let mut curr = &self.root;
        for ch in key.chars() {
            curr = curr.children.get(&ch)?;
        }
        curr.value.as_ref()
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn clear(&mut self) {
        self.root.children.clear();
        self.root.value = None;
        self.size = 0;
    }
}

impl<V> Default for Trie<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V> AlgorithmTrait for Trie<V> {
    fn name(&self) -> &'static str {
        "trie"
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn clear(&mut self) {
        self.clear();
    }
}

impl<V> TreeAlgorithmTrait<String, V> for Trie<V> {
    fn insert(&mut self, key: String, value: V) -> Option<V> {
        self.insert(&key, value)
    }

    fn get(&self, key: &String) -> Option<&V> {
        self.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        trie.insert("route/user", 100);
        trie.insert("route/auth", 200);

        assert_eq!(trie.get("route/user"), Some(&100));
        assert_eq!(trie.get("route/auth"), Some(&200));
        assert_eq!(trie.get("route/unknown"), None);
    }
}
