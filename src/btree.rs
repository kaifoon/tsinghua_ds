//! BTree
//! M way balance search tree

use crate::{BTNode, BST};

/// B Tree
#[derive(Debug, PartialEq, Eq)]
pub struct BTree {
    /// Root node
    pub root: Option<Box<BTNode>>,
    /// key code size
    pub size: usize,
    /// order size must >= 2
    pub order: usize,
    /// lower bound
    pub lower_bound: usize,
}

impl BTree {
    /// Create a upper bound [order / 2 - 1, order] B tree
    pub fn new(size: usize, order: usize) -> Self {
        assert!(order > 2, "order must greater than 2");

        let mut root = Box::new(BTNode::default());
        root.children.push(None);
        Self {
            root: Some(root),
            size,
            order,
            lower_bound: (order as f64 / 2.0).ceil() as usize,
        }
    }

    /// get size
    #[inline]
    pub fn get_size(&self) -> usize {
        self.size
    }
    /// get order
    #[inline]
    pub fn get_order(&self) -> usize {
        self.order
    }
    /// Overflow algorithm
    pub fn overflow(&mut self, mut ptrs: Vec<*mut BTNode>, mut node: &mut BTNode) {
        while let Some(parent_ptr) = ptrs.pop() {
            let median = node.keys.len() / 2;
            let right_keys = node.keys.split_off(median + 1);
            let right_vals = node.vals.split_off(median + 1);
            let right_children = node.children.split_off(median + 1);

            let (up_key, up_val) = (node.keys.pop().unwrap(), node.vals.pop().unwrap());

            let right_node = Some(Box::new(BTNode::new(
                right_keys,
                right_vals,
                right_children,
            )));

            let parent_node = unsafe { &mut *parent_ptr };
            let idx = parent_node.keys.binary_search(&up_key).unwrap_err();

            parent_node.keys.insert(idx, up_key);
            parent_node.vals.insert(idx, up_val);
            parent_node.children.insert(idx + 1, right_node);

            if parent_node.keys.len() <= self.order {
                break;
            }
            node = parent_node;
        }

        // root node split
        let root = self.root.as_mut().unwrap();
        if root.keys.len() < self.order {
            return;
        }

        let median = root.keys.len() / 2;
        let right_keys = root.keys.split_off(median + 1);
        let right_vals = root.vals.split_off(median + 1);
        let right_children = root.children.split_off(median + 1);

        let (up_key, up_val) = (root.keys.pop().unwrap(), root.vals.pop().unwrap());

        let right_node = Some(Box::new(BTNode::new(
            right_keys,
            right_vals,
            right_children,
        )));

        let mut new_root = Box::new(BTNode::default());
        new_root.keys.push(up_key);
        new_root.vals.push(up_val);
        new_root.children.push(self.root.take());
        new_root.children.push(right_node);

        self.root = Some(new_root);
    }

    /// Underflow algorithm
    pub fn underflow(&mut self, mut ptrs: Vec<*mut BTNode>, mut node: &mut BTNode) {
        while let Some(parent_ptr) = ptrs.pop() {
            let parent_node = unsafe { &mut *parent_ptr };
            let idx = parent_node
                .children
                .iter()
                .position(|child| {
                    if let Some(c) = child.as_ref() {
                        &**c == node
                    } else {
                        false
                    }
                })
                .unwrap();

            if idx == 0 {
                if let Some(right_sibling) = parent_node.children[idx + 1].as_mut() {
                    // rotate
                    if right_sibling.keys.len() + 1 > self.lower_bound {
                        node.keys.push(parent_node.keys[idx]);
                        node.vals.push(parent_node.vals[idx]);

                        parent_node.keys[idx] = right_sibling.keys.remove(0);
                        parent_node.vals[idx] = right_sibling.vals.remove(0);
                        // wrong
                        node.children.push(right_sibling.children.remove(0));
                    } else {
                        // merge
                        node.keys.push(parent_node.keys.remove(idx));
                        node.vals.push(parent_node.vals.remove(idx));

                        node.keys.append(&mut right_sibling.keys);
                        node.vals.append(&mut right_sibling.vals);
                        node.children.append(&mut right_sibling.children);

                        parent_node.children.remove(idx + 1);
                    }
                }
            } else if idx == parent_node.children.len() - 1 {
                if let Some(left_sibling) = parent_node.children[idx - 1].as_mut() {
                    // rotate
                    if left_sibling.keys.len() + 1 > self.lower_bound {
                        node.keys.insert(0, parent_node.keys[idx - 1]);
                        node.vals.insert(0, parent_node.vals[idx - 1]);

                        parent_node.keys[idx - 1] = left_sibling.keys.pop().unwrap();
                        parent_node.vals[idx - 1] = left_sibling.vals.pop().unwrap();
                        node.children.push(left_sibling.children.pop().unwrap());
                    } else {
                        // merge
                        left_sibling.keys.push(parent_node.keys.remove(idx - 1));
                        left_sibling.vals.push(parent_node.vals.remove(idx - 1));

                        left_sibling.keys.append(&mut node.keys);
                        left_sibling.vals.append(&mut node.vals);
                        left_sibling.children.append(&mut node.children);

                        parent_node.children.remove(idx);
                    }
                }
            } else {
                if let Some(left_sibling) = parent_node.children[idx - 1].as_mut() {
                    // rotate
                    if left_sibling.keys.len() + 1 > self.lower_bound {
                        node.keys.insert(0, parent_node.keys[idx - 1]);
                        node.vals.insert(0, parent_node.vals[idx - 1]);

                        parent_node.keys[idx - 1] = left_sibling.keys.pop().unwrap();
                        parent_node.vals[idx - 1] = left_sibling.vals.pop().unwrap();
                        node.children.push(left_sibling.children.pop().unwrap());
                    } else {
                        // merge
                        left_sibling.keys.push(parent_node.keys.remove(idx - 1));
                        left_sibling.vals.push(parent_node.vals.remove(idx - 1));

                        left_sibling.keys.append(&mut node.keys);
                        left_sibling.vals.append(&mut node.vals);
                        left_sibling.children.append(&mut node.children);

                        parent_node.children.remove(idx);
                    }
                // after left [19] [18,14,]
                // rotate
                } else if let Some(right_sibling) = parent_node.children[idx + 1].as_mut() {
                    // rotate
                    if right_sibling.keys.len() + 1 > self.lower_bound {
                        node.keys.push(parent_node.keys[idx - 1]);
                        node.vals.push(parent_node.vals[idx - 1]);

                        parent_node.keys[idx - 1] = right_sibling.keys.remove(0);
                        parent_node.vals[idx - 1] = right_sibling.vals.remove(0);
                        node.children.push(right_sibling.children.remove(0));
                    } else {
                        // merge
                        node.keys.push(parent_node.keys.remove(idx - 1));
                        node.vals.push(parent_node.vals.remove(idx - 1));

                        node.keys.append(&mut right_sibling.keys);
                        node.vals.append(&mut right_sibling.vals);
                        node.children.append(&mut right_sibling.children);

                        parent_node.children.remove(idx);
                    }
                }
            }

            if parent_node.keys.len() + 1 >= self.lower_bound {
                break;
            }

            node = parent_node;
        }

        let root = self.root.as_mut().unwrap();
        if root.keys.len() == 0 {
            let new_root = root.children[0].take();
            self.root = new_root;
        }
    }

    /// return a vector of pointer from root to key path
    fn searchin(&mut self, key: i32) -> Vec<*mut BTNode> {
        let mut ptr = self.root.as_mut();
        let mut prev_ptrs = Vec::<*mut BTNode>::new();

        while let Some(node) = ptr {
            prev_ptrs.push(&mut **node);

            let next_idx = match node.keys.binary_search(&key) {
                Ok(_) => break,
                Err(idx) => idx,
            };
            ptr = node.children[next_idx].as_mut();
        }

        prev_ptrs
    }
}

impl BST for BTree {
    /// search val by key
    fn search(&mut self, key: i32) -> Option<i32> {
        if let Some(node_ptr) = self.searchin(key).pop() {
            let node = unsafe { &mut *node_ptr };
            if let Ok(idx) = node.keys.binary_search(&key) {
                return Some(node.vals[idx]);
            }
        }

        None
    }

    /// insert key, value. if children more than order will split and overflow
    fn insert(&mut self, key: i32, val: i32) {
        let mut prev_ptrs = self.searchin(key);
        if let Some(node_ptr) = prev_ptrs.pop() {
            let node = unsafe { &mut *node_ptr };
            match node.keys.binary_search(&key) {
                Ok(idx) => {
                    node.vals[idx] = val;
                    return;
                }
                Err(idx) => {
                    node.keys.insert(idx, key);
                    node.vals.insert(idx, val);

                    node.children.push(None);
                    self.size += 1;
                    // if key size more equal than order
                    // need to overflow
                    if self.order == node.keys.len() {
                        self.overflow(prev_ptrs, node);
                    }
                }
            }
        }
    }

    /// remove val
    fn remove(&mut self, key: i32) -> Option<i32> {
        let mut prev_ptrs = self.searchin(key);
        if let Some(node_ptr) = prev_ptrs.pop() {
            let node = unsafe { &mut *node_ptr };
            if let Ok(idx) = node.keys.binary_search(&key) {
                let mut ptr = node.children[idx + 1].as_mut();

                // node is leaf node
                if ptr.is_none() {
                    node.keys.remove(idx);
                    node.children.pop();
                    let inner_val = node.vals.remove(idx);

                    if self.root.as_ref().unwrap().keys.is_empty() {
                        return Some(inner_val);
                    }

                    if self.lower_bound == node.keys.len() + 2 {
                        self.underflow(prev_ptrs, node);
                    }

                    return Some(inner_val);
                }

                // node is a branch, need to find leaf
                prev_ptrs.push(node_ptr);
                while let Some(inner_node) = ptr {
                    prev_ptrs.push(&mut **inner_node);
                    ptr = inner_node.children[0].as_mut();
                }

                if let Some(inner_ptr) = prev_ptrs.pop() {
                    let inner_node = unsafe { &mut *inner_ptr };
                    let inner_val = Some(node.vals[idx]);

                    node.keys[idx] = inner_node.keys[0];
                    node.vals[idx] = inner_node.vals[0];

                    inner_node.keys.remove(0);
                    inner_node.vals.remove(0);
                    inner_node.children.pop();
                    self.size -= 1;

                    if self.root.as_ref().unwrap().keys.is_empty() {
                        return inner_val;
                    }

                    if self.lower_bound == inner_node.keys.len() + 2 {
                        self.underflow(prev_ptrs, inner_node);
                    }
                    return inner_val;
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tree_operations() {
        for order in 3..10 {
            let mut bst = BTree::new(0, order);
            bst.insert(4, 16);
            assert_eq!(bst.search(4), Some(16));
            assert_eq!(bst.remove(4), Some(16));
            bst.insert(8, 256);

            bst.insert(5, 32);
            bst.insert(9, 512);
            bst.insert(27, 28499);
            bst.insert(3, 8);
            bst.insert(2, 4);
            bst.insert(31, 28499);
            bst.insert(25, 28499);
            bst.insert(15, 12345);
            bst.insert(13, 2344);
            bst.insert(23, 28499);
            bst.insert(11, 234);
            bst.insert(18, 1994);
            bst.insert(20, 12993);
            bst.insert(19, 2849);
            bst.insert(12, 28499);
            bst.insert(32, 28499);
            bst.insert(26, 28499);
            bst.insert(10, 28499);
            bst.insert(28, 28499);
            bst.insert(1, 28499);
            bst.insert(6, 28499);
            bst.insert(14, 28499);
            bst.insert(30, 28499);
            bst.insert(24, 28499);
            bst.insert(28, 28499);
            bst.insert(22, 28499);
            bst.insert(29, 28499);

            assert_eq!(bst.search(8), Some(256));
            assert_eq!(bst.search(5), Some(32));
            assert_eq!(bst.search(9), Some(512));
            assert_eq!(bst.search(3), Some(8));
            assert_eq!(bst.search(2), Some(4));
            assert_eq!(bst.search(15), Some(12345));
            assert_eq!(bst.search(13), Some(2344));
            assert_eq!(bst.search(11), Some(234));
            assert_eq!(bst.search(18), Some(1994));
            assert_eq!(bst.search(20), Some(12993));
            assert_eq!(bst.search(19), Some(2849));
            assert_eq!(bst.search(12), Some(28499));
            assert_eq!(bst.search(10), Some(28499));
            assert_eq!(bst.search(1), Some(28499));
            assert_eq!(bst.search(6), Some(28499));
            assert_eq!(bst.search(14), Some(28499));
            assert_eq!(bst.search(24), Some(28499));
            assert_eq!(bst.search(28), Some(28499));
            assert_eq!(bst.search(22), Some(28499));
            assert_eq!(bst.search(23), Some(28499));
            assert_eq!(bst.search(25), Some(28499));
            assert_eq!(bst.search(26), Some(28499));
            assert_eq!(bst.remove(2), Some(4));
            assert_eq!(bst.remove(11), Some(234));
            assert_eq!(bst.remove(20), Some(12993));
            assert_eq!(bst.remove(8), Some(256));
            assert_eq!(bst.remove(15), Some(12345));

            assert_eq!(bst.search(9), Some(512));
            assert_eq!(bst.search(19), Some(2849));
            assert_eq!(bst.remove(10), Some(28499));
            assert_eq!(bst.remove(5), Some(32));
            assert_eq!(bst.remove(9), Some(512));
            assert_eq!(bst.remove(3), Some(8));
            assert_eq!(bst.search(22), Some(28499));
            bst.insert(19, -12849);
            assert_eq!(bst.remove(1), Some(28499));
            assert_eq!(bst.remove(6), Some(28499));
            assert_eq!(bst.remove(13), Some(2344));
            assert_eq!(bst.remove(12), Some(28499));
            assert_eq!(bst.remove(19), Some(-12849));
            assert_eq!(bst.remove(18), Some(1994));
            assert_eq!(bst.remove(22), Some(28499));
            assert_eq!(bst.remove(23), Some(28499));
            assert_eq!(bst.remove(24), Some(28499));
            assert_eq!(bst.remove(25), Some(28499));
            assert_eq!(bst.remove(26), Some(28499));
            assert_eq!(bst.remove(27), Some(28499));
            assert_eq!(bst.remove(28), Some(28499));
            assert_eq!(bst.remove(29), Some(28499));
            assert_eq!(bst.remove(14), Some(28499));
            assert_eq!(bst.remove(30), Some(28499));
            assert_eq!(bst.remove(31), Some(28499));
            assert_eq!(bst.remove(32), Some(28499));
        }
    }
}
