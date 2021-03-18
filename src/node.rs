//! Common Node Defination
use std::cmp::max;
use std::mem::{replace, swap};

/// Common Tree Node
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    /// key
    pub key: i32,
    /// value
    pub val: i32,
    /// height
    pub height: usize,
    /// left child
    pub left: Option<Box<TreeNode>>,
    /// right child
    pub right: Option<Box<TreeNode>>,
}
/// TreeNode Child Type
#[derive(Debug, PartialEq, Eq)]
pub enum ChildType {
    Left,
    Right,
}

impl TreeNode {
    /// New treenode instance
    #[inline]
    pub const fn new(key: i32, val: i32) -> Self {
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
                if left_node.balance_factor() == -1 {
                    left_node.rotate_left();
                }

                self.rotate_right();
            }
            _ => (),
        }
    }

    /// caller `self` must be `parent`'s child
    fn who_child(&self, parent: &TreeNode) -> ChildType {
        if let Some(node) = parent.left.as_ref() {
            if node.key == self.key {
                return ChildType::Left;
            }
        }

        return ChildType::Right;
    }

    /// splay algorithm
    pub fn splay(mut node: &mut TreeNode, mut prev_ptrs: Vec<*mut TreeNode>) {
        let last_parent = if prev_ptrs.len() % 2 == 1 {
            prev_ptrs.remove(0)
        } else {
            0 as *mut TreeNode
        };

        while let (Some(parent_ptr), Some(grandparent_ptr)) = (prev_ptrs.pop(), prev_ptrs.pop()) {
            let grandparent = unsafe { &mut *grandparent_ptr };
            let parent = unsafe { &mut *parent_ptr };

            match (node.who_child(parent), parent.who_child(grandparent)) {
                (ChildType::Left, ChildType::Left) => {
                    // zig-zig
                    let v_left = node.left.take();
                    let v_right = node.right.take();

                    let p_left = parent.left.take();
                    let p_right = parent.right.take();

                    let g_left = grandparent.left.take();
                    let g_right = grandparent.right.take();

                    // grandparent swap with self
                    swap(&mut node.key, &mut grandparent.key);
                    swap(&mut node.val, &mut grandparent.val);

                    node.left = p_right;
                    node.right = g_right;

                    parent.left = v_right;
                    parent.right = p_left;

                    grandparent.left = v_left;
                    grandparent.right = g_left;

                    node.update_height();
                    parent.update_height();
                    grandparent.update_height();

                    node = grandparent;
                }
                (ChildType::Right, ChildType::Right) => {
                    // zag-zag
                    let v_left = node.left.take();
                    let v_right = node.right.take();

                    let p_left = parent.left.take();
                    let p_right = parent.right.take();

                    let g_left = grandparent.left.take();
                    let g_right = grandparent.right.take();

                    // grandparent swap with self
                    swap(&mut node.key, &mut grandparent.key);
                    swap(&mut node.val, &mut grandparent.val);

                    node.left = g_left;
                    node.right = p_left;

                    parent.right = v_left;
                    parent.left = p_right;

                    grandparent.right = v_right;
                    grandparent.left = g_right;

                    node.update_height();
                    parent.update_height();
                    grandparent.update_height();

                    node = grandparent;
                }
                (ChildType::Left, ChildType::Right) => {
                    // zig-zag
                    let v_left = node.left.take();
                    let v_right = node.right.take();

                    let p_left = parent.left.take();
                    let g_left = grandparent.left.take();

                    // grandparent swap with self
                    swap(&mut node.key, &mut grandparent.key);
                    swap(&mut node.val, &mut grandparent.val);

                    node.left = g_left;
                    node.right = v_left;

                    parent.left = v_right;
                    grandparent.left = p_left;

                    node.update_height();
                    parent.update_height();
                    grandparent.update_height();

                    node = grandparent;
                }
                (ChildType::Right, ChildType::Left) => {
                    // zag-zig
                    let v_left = node.left.take();
                    let v_right = node.right.take();

                    let p_right = parent.right.take();
                    let g_right = grandparent.right.take();

                    // grandparent swap with self
                    swap(&mut node.key, &mut grandparent.key);
                    swap(&mut node.val, &mut grandparent.val);

                    node.left = v_right;
                    node.right = g_right;

                    parent.right = v_left;
                    grandparent.right = p_right;

                    node.update_height();
                    parent.update_height();
                    grandparent.update_height();

                    node = grandparent;
                }
            }
        }
        // do another single rotatation
        if !last_parent.is_null() {
            // grand grand parent
            let parent = unsafe { &mut *last_parent };
            match node.who_child(parent) {
                ChildType::Left => {
                    parent.rotate_right();
                }
                ChildType::Right => {
                    parent.rotate_left();
                }
            }
        }
    }
}
