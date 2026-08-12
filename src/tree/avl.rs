use crate::abstraction::AlgorithmTrait;
use super::abstraction::TreeAlgorithmTrait;

struct AvlNode<K, V> {
    key: K,
    value: V,
    height: i32,
    left: Option<Box<AvlNode<K, V>>>,
    right: Option<Box<AvlNode<K, V>>>,
}

impl<K, V> AvlNode<K, V> {
    fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            height: 1,
            left: None,
            right: None,
        }
    }

    fn height(node: Option<&Box<AvlNode<K, V>>>) -> i32 {
        node.map_or(0, |n| n.height)
    }

    fn update_height(&mut self) {
        self.height = 1 + Self::height(self.left.as_ref()).max(Self::height(self.right.as_ref()));
    }

    fn balance_factor(&self) -> i32 {
        Self::height(self.left.as_ref()) - Self::height(self.right.as_ref())
    }
}

pub struct AvlTree<K, V> {
    root: Option<Box<AvlNode<K, V>>>,
    size: usize,
}

impl<K: Ord, V> AvlTree<K, V> {
    pub fn new() -> Self {
        Self { root: None, size: 0 }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let (root, old) = Self::insert_node(self.root.take(), key, value);
        self.root = root;
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
        node: Option<Box<AvlNode<K, V>>>,
        key: K,
        value: V,
    ) -> (Option<Box<AvlNode<K, V>>>, Option<V>) {
        let mut node = match node {
            None => return (Some(Box::new(AvlNode::new(key, value))), None),
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

        node.update_height();
        (Self::rebalance(node), old_val)
    }

    fn rebalance(mut node: Box<AvlNode<K, V>>) -> Option<Box<AvlNode<K, V>>> {
        let bf = node.balance_factor();
        if bf > 1 {
            if node.left.as_ref().map_or(0, |n| n.balance_factor()) < 0 {
                node.left = Self::rotate_left(node.left.take().unwrap());
            }
            return Self::rotate_right(node);
        }
        if bf < -1 {
            if node.right.as_ref().map_or(0, |n| n.balance_factor()) > 0 {
                node.right = Self::rotate_right(node.right.take().unwrap());
            }
            return Self::rotate_left(node);
        }
        Some(node)
    }

    fn rotate_left(mut node: Box<AvlNode<K, V>>) -> Option<Box<AvlNode<K, V>>> {
        let mut right = node.right.take().unwrap();
        node.right = right.left.take();
        node.update_height();
        right.left = Some(node);
        right.update_height();
        Some(right)
    }

    fn rotate_right(mut node: Box<AvlNode<K, V>>) -> Option<Box<AvlNode<K, V>>> {
        let mut left = node.left.take().unwrap();
        node.left = left.right.take();
        node.update_height();
        left.right = Some(node);
        left.update_height();
        Some(left)
    }
}

impl<K: Ord, V> Default for AvlTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> AlgorithmTrait for AvlTree<K, V> {
    fn name(&self) -> &'static str {
        "avl_tree"
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

impl<K: Ord, V> TreeAlgorithmTrait<K, V> for AvlTree<K, V> {
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
    fn test_avl_tree() {
        let mut tree = AvlTree::new();
        tree.insert(10, "ten");
        tree.insert(20, "twenty");
        tree.insert(30, "thirty");

        assert_eq!(tree.get(&20), Some(&"twenty"));
        assert_eq!(tree.len(), 3);
    }
}
