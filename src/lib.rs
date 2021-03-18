//! TsingHua DataStructure Lesson Advanced BST implementation

#![deny(future_incompatible, nonstandard_style, warnings, missing_docs)]

pub use avl::AVLTree;
pub use binary::BinarySearchTree;
pub use node::TreeNode;
pub use splay::SplayTree;
mod avl;
mod binary;
mod node;
mod splay;

/// Binary Search Tree Interface
pub trait BST {
    /// search val by key
    fn search(&mut self, key: i32) -> Option<i32>;
    /// insert key-val  
    fn insert(&mut self, key: i32, val: i32);
    /// remove val by key
    fn remove(&mut self, key: i32) -> Option<i32>;
}
