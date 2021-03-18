//! SplayTree

use crate::{TreeNode, BST};
use std::mem::replace;

/// Splay Tree
#[derive(Debug, PartialEq, Eq)]
pub struct SplayTree(Option<Box<TreeNode>>);

impl SplayTree {
    /// Splay tree
    pub fn new(root_key: i32, root_val: i32) -> Self {
        Self(Some(Box::new(TreeNode::new(root_key, root_val))))
    }

    fn searchin(&mut self, key: i32) {
        let mut ptr = self.0.as_mut();
        let mut prev_ptrs = Vec::<*mut TreeNode>::new();

        while let Some(node) = ptr {
            let node_key = node.key;
            prev_ptrs.push(&mut **node);
            if key > node_key {
                ptr = node.right.as_mut();
            } else if key < node_key {
                ptr = node.left.as_mut();
            } else {
                break;
            }
        }

        if let Some(node_ptr) = prev_ptrs.pop() {
            let node = unsafe { &mut *node_ptr };
            node.splay(prev_ptrs);
        }
    }
}

impl BST for SplayTree {
    /// search val by key
    fn search(&mut self, key: i32) -> Option<i32> {
        self.searchin(key);

        if let Some(node) = self.0.as_ref() {
            // found proper key
            if node.key == key {
                return Some(node.val);
            }
        }
        // Tree is empty or found nearest key
        None
    }
    /// 3-4 insert method rotated
    fn insert(&mut self, key: i32, val: i32) {
        self.searchin(key);
        if let Some(node) = self.0.as_mut() {
            if node.key == key {
                node.val = val;
            } else if node.key < key {
                let old_key = replace(&mut node.key, key);
                let old_val = replace(&mut node.val, val);

                let mut old_node = Box::new(TreeNode::new(old_key, old_val));
                old_node.left = node.left.take();
                old_node.update_height();
                node.left = Some(old_node);
                node.update_height();
            } else {
                let old_key = replace(&mut node.key, key);
                let old_val = replace(&mut node.val, val);

                let mut old_node = Box::new(TreeNode::new(old_key, old_val));
                old_node.right = node.right.take();
                old_node.update_height();
                node.right = Some(old_node);
                node.update_height();
            }
        }
    }
    /// remove val
    fn remove(&mut self, key: i32) -> Option<i32> {
        self.searchin(key);

        if let Some(node) = self.0.as_mut() {
            if node.key == key {
                if node.right.is_none() {
                    let left_tree = node.left.take();
                    let inner_val = node.val;
                    self.0 = left_tree;
                    return Some(inner_val);
                }

                // if right.left.is_none(), min right child is right node, so
                // take right
                if node.right.as_ref().unwrap().left.is_none() {
                    let mut right_node = node.right.take().unwrap();
                    let inner_val = replace(&mut node.val, right_node.val);
                    let _ = replace(&mut node.key, right_node.key);
                    let _ = replace(&mut node.right, right_node.right.take());
                    node.update_height();
                    return Some(inner_val);
                }

                let mut right_tree = node.right.as_mut();
                let mut parent_left_ptr: *mut TreeNode = 0 as *mut TreeNode;
                while let Some(next_node) = right_tree {
                    if next_node.left.is_some() {
                        parent_left_ptr = &mut **next_node;
                    }

                    right_tree = next_node.left.as_mut();
                }

                let parent_left_node = unsafe { &mut *parent_left_ptr };
                let mut leftmost_node = parent_left_node.left.take().unwrap();

                // replace key, value
                let inner_val = replace(&mut node.val, leftmost_node.val);
                let _ = replace(&mut node.key, leftmost_node.key);
                let _ = replace(&mut parent_left_node.left, leftmost_node.right.take());
                node.update_height();
                return Some(inner_val);
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
        let mut bst = SplayTree::new(10, 1024);

        bst.insert(4, 16);
        bst.insert(8, 256);
        bst.insert(5, 32);
        bst.insert(9, 512);
        bst.insert(3, 8);
        bst.insert(2, 4);
        bst.insert(15, 12345);
        println!();
        bst.insert(13, 2344);
        bst.insert(11, 234);
        bst.insert(18, 1994);
        bst.insert(20, 12993);
        bst.insert(19, 2849);
        bst.insert(12, 28499);

        assert_eq!(bst.search(4), Some(16));
        assert_eq!(bst.search(19), Some(2849));
        bst.insert(19, -12849);
        assert_eq!(bst.search(19), Some(-12849));
        assert_eq!(bst.search(13), Some(2344));
        assert_eq!(bst.search(4), Some(16));
        assert_eq!(bst.search(-1), None);
        assert_eq!(bst.search(0), None);
        assert_eq!(bst.remove(2), Some(4));
        assert_eq!(bst.search(2), None);
        assert_eq!(bst.remove(2), None);
        assert_eq!(bst.remove(11), Some(234));
        assert_eq!(bst.remove(11), None);
        assert_eq!(bst.search(11), None);
        assert_eq!(bst.remove(20), Some(12993));
        assert_eq!(bst.remove(20), None);
        assert_eq!(bst.search(20), None);
        assert_eq!(bst.remove(8), Some(256));
        assert_eq!(bst.remove(8), None);
        assert_eq!(bst.search(8), None);
        assert_eq!(bst.search(12), Some(28499));
        bst.insert(17, 49);
        bst.insert(14, 149);
        bst.insert(16, 169);
        assert_eq!(bst.remove(15), Some(12345));
        assert_eq!(bst.remove(15), None);
        assert_eq!(bst.search(15), None);
        bst.insert(15, 12345);
    }
}
