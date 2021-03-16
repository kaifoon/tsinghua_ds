//! TsingHua DataStructure Lesson Advanced BST implementation

#![deny(future_incompatible, nonstandard_style, warnings, missing_docs)]

pub use avl::AVLTree;
pub use binary::BinarySearchTree;
mod avl;
mod binary;
use std::cmp::max;
use std::mem::{replace, swap};

/// Binary Search Tree Interface
pub trait BST {
    /// search val by key
    fn search(&self, key: i32) -> Option<i32>;
    /// insert key-val  
    fn insert(&mut self, key: i32, val: i32);
    /// remove val by key
    fn remove(&mut self, key: i32) -> Option<i32>;
}

/// Common TreeNode
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    key: i32,
    val: i32,
    height: usize,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    /// New treenode instance
    #[inline]
    pub fn new(key: i32, val: i32) -> Self {
        Self {
            key,
            val,
            height: 1,
            left: None,
            right: None,
        }
    }
    /// get key
    #[inline]
    pub fn get_key(&self) -> i32 {
        self.key
    }

    /// get val
    #[inline]
    pub fn get_val(&self) -> i32 {
        self.val
    }
    /// set val
    #[inline]
    pub fn set_val(&mut self, val: i32) {
        self.val = val;
    }

    /// get left child height
    #[inline]
    fn left_height(&self) -> usize {
        self.left.as_ref().map_or(0, |left| left.height)
    }
    /// get right child height
    #[inline]
    fn right_height(&self) -> usize {
        self.right.as_ref().map_or(0, |right| right.height)
    }
    /// update `TreeNode
    #[inline]
    pub fn update_height(&mut self) {
        self.height = 1 + max(self.left_height(), self.right_height());
    }
    /// get balance factor,
    /// TreeHeavy::Left mean left child heavy, so call `rotate_right()` method
    /// TreeHeavy::Right mean right child heavy, so call `rotate_left()` method
    fn balance_factor(&self) -> i8 {
        let left_height = self.left_height();
        let right_height = self.right_height();

        if left_height >= right_height {
            (left_height - right_height) as i8
        } else {
            -((right_height - left_height) as i8)
        }
    }

    /// Rotate left , mean `zag`
    fn rotate_left(&mut self) {
        if self.right.is_none() {
            return;
        }

        // right node must not be `None`, because right heavy
        let right_node = self.right.as_mut().unwrap();
        let right_left_tree = right_node.left.take();
        let right_right_tree = right_node.right.take();

        let mut new_left_tree = replace(&mut self.right, right_right_tree);
        swap(&mut self.val, &mut new_left_tree.as_mut().unwrap().val);
        swap(&mut self.key, &mut new_left_tree.as_mut().unwrap().key);

        let left_tree = self.left.take();

        let new_left_node = new_left_tree.as_mut().unwrap();
        new_left_node.right = right_left_tree;
        new_left_node.left = left_tree;
        self.left = new_left_tree;

        if let Some(node) = self.left.as_mut() {
            node.update_height();
        }

        self.update_height();
    }
    /// Rotate right , mean `zig`
    fn rotate_right(&mut self) {
        if self.left.is_none() {
            return;
        }
        // left node must not be `None`, because left heavy
        let left_node = self.left.as_mut().unwrap();
        let left_right_tree = left_node.right.take();
        let left_left_tree = left_node.left.take();

        let mut new_right_tree = replace(&mut self.left, left_left_tree);
        swap(&mut self.val, &mut new_right_tree.as_mut().unwrap().val);
        swap(&mut self.key, &mut new_right_tree.as_mut().unwrap().key);
        let right_tree = self.right.take();

        let new_right_node = new_right_tree.as_mut().unwrap();
        new_right_node.left = left_right_tree;
        new_right_node.right = right_tree;
        self.right = new_right_tree;

        if let Some(node) = self.right.as_mut() {
            node.update_height();
        }

        self.update_height();
    }

    /// Rebalance tree
    pub fn rebalance(&mut self) {
        match self.balance_factor() {
            -2 => {
                // root is right heavy
                let right_node = self.right.as_mut().unwrap();
                // inner node is left heavy
                if right_node.balance_factor() == 1 {
                    right_node.rotate_right();
                }

                self.rotate_left();
            }
            2 => {
                // root is left heavy
                let left_node = self.left.as_mut().unwrap();
                // inner node is right heavy
                if left_node.balance_factor() == 1 {
                    left_node.rotate_left();
                }

                self.rotate_right();
            }
            _ => (),
        }
    }
}
