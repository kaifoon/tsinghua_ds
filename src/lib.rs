//! TsingHua DataStructure Lesson Advanced Data Structure implementation

#![doc(html_playground_url = "https://play.rust-lang.org/")]
#![deny(future_incompatible, nonstandard_style, warnings, missing_docs)]

pub use avl::AVLTree;
pub use binary::BinarySearchTree;
pub use btree::BTree;
pub use hashmap::HashMap;
pub use heap::{heap_sort, BinaryHeap, LeftistHeap};
pub use node::{BTNode, TreeNode};
pub use sort::*;
pub use splay::SplayTree;
pub use string::{bmbc_match, karp_rabin, kmp_match};
mod avl;
mod binary;
mod btree;
mod hashmap;
mod heap;
mod node;
mod sort;
mod splay;
mod string;

/// Binary Search Tree Interface
pub trait BST {
    /// search val by key
    fn search(&mut self, key: i32) -> Option<i32>;
    /// insert key-val  
    fn insert(&mut self, key: i32, val: i32);
    /// remove val by key
    fn remove(&mut self, key: i32) -> Option<i32>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn just_for_fun() {
        let mut array = [3, 4, 5, 2, 48, 19, 39, 390, 43, 12, 8, 9, 10, 7, 5];

        fn insert_sort(a: &mut [i32; 15]) {
            for i in 1..a.len() {
                let tmp = a[i];
                let mut idx = i;
                for j in (1..=i).rev() {
                    if tmp > a[j - 1] {
                        break;
                    }
                    a[j] = a[j - 1];
                    idx = j - 1;
                }
                a[idx] = tmp;
            }
        }

        insert_sort(&mut array);

        assert_eq!([2, 3, 4, 5, 5, 7, 8, 9, 10, 12, 19, 39, 43, 48, 390], array);
    }
}
