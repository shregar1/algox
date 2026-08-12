use crate::abstraction::AlgorithmTrait;
use super::abstraction::TreeAlgorithmTrait;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Color {
    Red,
    Black,
}

struct RbNode<K, V> {
    key: K,
    value: V,
    color: Color,
    left: Option<Box<RbNode<K, V>>>,
    right: Option<Box<RbNode<K, V>>>,
}

impl<K, V> RbNode<K, V> {
    fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            color: Color::Red,
            left: None,
            right: None,
        }
    }

    fn is_red(node: Option<&Box<RbNode<K, V>>>) -> bool {
        node.map_or(false, |n| n.color == Color::Red)
    }
}

pub struct RedBlackTree<K, V> {
    root: Option<Box<RbNode<K, V>>>,
    size: usize,
}

impl<K: Ord, V> RedBlackTree<K, V> {
    pub fn new() -> Self {
        Self { root: None, size: 0 }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let (root, old) = Self::insert_node(self.root.take(), key, value);
        let mut root = root.unwrap();
        root.color = Color::Black;
        self.root = Some(root);
        if old.is_none() {
            self.size += 1;
        }
        old
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut curr = self.root.as_ref();
        while let Some(node) = curr {
            match key.cmp(&node.key) {
                std::cmp::Ordering::Equal => return Some(&node.value),
                std::cmp::Ordering::Less => curr = node.left.as_ref(),
                std::cmp::Ordering::Greater => curr = node.right.as_ref(),
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn clear(&mut self) {
        self.root = None;
        self.size = 0;
    }

    fn insert_node(
        node: Option<Box<RbNode<K, V>>>,
        key: K,
        value: V,
    ) -> (Option<Box<RbNode<K, V>>>, Option<V>) {
        let mut node = match node {
            None => return (Some(Box::new(RbNode::new(key, value))), None),
            Some(n) => n,
        };

        let old_val;
        match key.cmp(&node.key) {
            std::cmp::Ordering::Less => {
                let (left, old) = Self::insert_node(node.left.take(), key, value);
                node.left = left;
                old_val = old;
            }
            std::cmp::Ordering::Greater => {
                let (right, old) = Self::insert_node(node.right.take(), key, value);
                node.right = right;
                old_val = old;
            }
            std::cmp::Ordering::Equal => {
                let old = std::mem::replace(&mut node.value, value);
                return (Some(node), Some(old));
            }
        }

        if RbNode::is_red(node.right.as_ref()) && !RbNode::is_red(node.left.as_ref()) {
            node = Self::rotate_left(node);
        }
        if RbNode::is_red(node.left.as_ref())
            && RbNode::is_red(node.left.as_ref().unwrap().left.as_ref())
        {
            node = Self::rotate_right(node);
        }
        if RbNode::is_red(node.left.as_ref()) && RbNode::is_red(node.right.as_ref()) {
            Self::flip_colors(&mut node);
        }

        (Some(node), old_val)
    }

    fn rotate_left(mut node: Box<RbNode<K, V>>) -> Box<RbNode<K, V>> {
        let mut right = node.right.take().unwrap();
        node.right = right.left.take();
        right.color = node.color;
        node.color = Color::Red;
        right.left = Some(node);
        right
    }

    fn rotate_right(mut node: Box<RbNode<K, V>>) -> Box<RbNode<K, V>> {
        let mut left = node.left.take().unwrap();
        node.left = left.right.take();
        left.color = node.color;
        node.color = Color::Red;
        left.right = Some(node);
        left
    }

    fn flip_colors(node: &mut RbNode<K, V>) {
        node.color = Color::Red;
        if let Some(left) = node.left.as_mut() {
            left.color = Color::Black;
        }
        if let Some(right) = node.right.as_mut() {
            right.color = Color::Black;
        }
    }
}

impl<K: Ord, V> Default for RedBlackTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> AlgorithmTrait for RedBlackTree<K, V> {
    fn name(&self) -> &'static str {
        "red_black_tree"
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

impl<K: Ord, V> TreeAlgorithmTrait<K, V> for RedBlackTree<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red_black_tree() {
        let mut tree = RedBlackTree::new();
        tree.insert(5, "five");
        tree.insert(2, "two");
        tree.insert(8, "eight");

        assert_eq!(tree.get(&5), Some(&"five"));
        assert_eq!(tree.len(), 3);
    }
}
