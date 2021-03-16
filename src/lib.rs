//! TsingHua DataStructure Lesson BST implementation

#![deny(future_incompatible, nonstandard_style, warnings, missing_docs)]

pub use avl::AVLTree;
pub use binary::BinarySearchTree;
mod avl;
mod binary;

/// Common TreeNode
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    key: i32,
    val: i32,
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
            left: None,
            right: None,
        }
    }
    /// get key
    pub fn get_key(&self) -> i32 {
        self.key
    }

    /// get val
    pub fn get_val(&self) -> i32 {
        self.val
    }
    /// set val
    pub fn set_val(&mut self, val: i32) {
        self.val = val;
    }
}

/// Binary Search Tree Interface
pub trait BST {
    /// search val by key
    fn search(&self, key: i32) -> Option<i32>;
    /// insert key-val  
    fn insert(&mut self, key: i32, val: i32);
    /// remove val by key
    fn remove(&mut self, key: i32) -> Option<i32>;
}
