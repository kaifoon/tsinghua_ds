//! BinarySearchTree

use crate::{TreeNode, BST};
use std::mem::replace;

/// BinarySearch Tree
#[derive(Debug, PartialEq, Eq)]
pub struct BinarySearchTree(Option<Box<TreeNode>>);

impl BinarySearchTree {
    /// BinarySearch tree
    pub fn new(root_key: i32, root_val: i32) -> Self {
        Self(Some(Box::new(TreeNode::new(root_key, root_val))))
    }
}

impl BST for BinarySearchTree {
    /// search val by key
    fn search(&mut self, key: i32) -> Option<i32> {
        let mut ptr = self.0.as_ref();
        while let Some(node) = ptr {
            let node_key = node.get_key();
            if key > node_key {
                ptr = node.right.as_ref();
            } else if key < node_key {
                ptr = node.left.as_ref();
            } else if key == node_key {
                return Some(node.get_val());
            }
        }
        None
    }
    /// 3-4 insert method rotated
    fn insert(&mut self, key: i32, val: i32) {
        let mut ptr = self.0.as_mut();
        while let Some(node) = ptr {
            let node_key = node.get_key();
            if key > node_key {
                if node.right.is_none() {
                    node.right = Some(Box::new(TreeNode::new(key, val)));
                    break;
                }
                ptr = node.right.as_mut();
            } else if key < node_key {
                if node.left.is_none() {
                    node.left = Some(Box::new(TreeNode::new(key, val)));
                    break;
                }
                ptr = node.left.as_mut();
            } else {
                node.set_val(val);
                break;
            }
        }
    }
    /// remove val
    fn remove(&mut self, key: i32) -> Option<i32> {
        let mut ptr = self.0.as_mut();
        let mut prev_ptrs = Vec::<*mut TreeNode>::new();
        let mut target_val = None;

        while let Some(node) = ptr {
            let node_key = node.get_key();
            if key > node_key {
                prev_ptrs.push(&mut **node);
                ptr = node.right.as_mut();
            } else if key < node_key {
                prev_ptrs.push(&mut **node);
                ptr = node.left.as_mut();
            } else {
                target_val = Some(&mut **node);
                break;
            }
        }

        match target_val {
            None => None,
            Some(node) => {
                // two children
                if node.left.is_some() && node.right.is_some() {
                    // if right.left.is_none(), min right child is right node, so
                    // take right
                    if node.right.as_ref().unwrap().left.is_none() {
                        let mut right_node = node.right.take().unwrap();
                        let inner_val = replace(&mut node.val, right_node.val);
                        let _ = replace(&mut node.key, right_node.key);
                        let _ = replace(&mut node.right, right_node.right.take());
                        return Some(inner_val);
                    }

                    let mut rightmin = node.right.as_mut();
                    let mut inner_ptrs = Vec::<*mut TreeNode>::new();
                    while let Some(next_node) = rightmin {
                        if next_node.left.is_some() {
                            inner_ptrs.push(&mut **next_node);
                        }

                        rightmin = next_node.left.as_mut();
                    }

                    let parent_left_node = unsafe { &mut *inner_ptrs.pop().unwrap() };
                    let mut leftmost_node = parent_left_node.left.take().unwrap();

                    // replace key, value
                    let inner_val = replace(&mut node.val, leftmost_node.val);
                    let _ = replace(&mut node.key, leftmost_node.key);
                    let _ = replace(&mut parent_left_node.left, leftmost_node.right.take());
                    return Some(inner_val);

                // one or zero children
                } else if node.left.is_none() && node.right.is_some() {
                    let right_node = node.right.take().unwrap();
                    let inner_val = replace(node, *right_node).val;
                    return Some(inner_val);
                } else if node.right.is_none() && node.left.is_some() {
                    let left_node = node.left.take().unwrap();
                    let inner_val = replace(node, *left_node).val;
                    return Some(inner_val);
                } else {
                    if let Some(prev_ptr) = prev_ptrs.pop() {
                        let prev_node = unsafe { &mut *prev_ptr };
                        let inner_val = if let Some(left_node) = prev_node.left.as_ref() {
                            if left_node.val == node.val {
                                prev_node.left.take().unwrap().val
                            } else {
                                prev_node.right.take().unwrap().val
                            }
                        } else {
                            prev_node.right.take().unwrap().val
                        };

                        return Some(inner_val);
                    } else {
                        let inner_val = self.0.take().unwrap().val;
                        return Some(inner_val);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tree_operations() {
        let mut bst = BinarySearchTree::new(10, 1024);

        bst.insert(4, 16);
        bst.insert(8, 256);
        bst.insert(5, 32);
        bst.insert(9, 512);
        bst.insert(3, 8);
        bst.insert(2, 4);
        bst.insert(15, 12345);
        bst.insert(13, 2344);
        bst.insert(11, 234);
        bst.insert(18, 1994);
        bst.insert(20, 12993);
        bst.insert(19, 2849);
        bst.insert(12, 28499);

        assert_eq!(bst.search(9), Some(512));
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
        bst.insert(17, 49);
        bst.insert(14, 149);
        bst.insert(16, 169);
        assert_eq!(bst.remove(15), Some(12345));
        assert_eq!(bst.remove(15), None);
        assert_eq!(bst.search(15), None);
    }
}
