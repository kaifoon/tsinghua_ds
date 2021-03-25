//! A priority queue implemented with a binary heap.
//!
//! Insertion and popping the largest element have *O*(log(*n*)) time complexity.
//! Checking the largest element is *O*(1). Converting a vector to a binary heap
//! can be done in-place, and has *O*(*n*) complexity. A binary heap can also be
//! converted to a sorted vector in-place, allowing it to be used for an *O*(*n* \* log(*n*))
//! in-place heapsort.
//!
use std::mem::{swap, take};
/// A priority queue implemented with a binary heap.
///
/// This will be a max-heap.
///
/// It is a logic error for an item to be modified in such a way that the
/// item's ordering relative to any other item, as determined by the `Ord`
/// trait, changes while it is in the heap. This is normally only possible
/// through `Cell`, `RefCell`, global state, I/O, or unsafe code.
///
/// # Examples
///
/// ```
/// use tsinghua_ds::BinaryHeap;
///
/// // Type inference lets us omit an explicit type signature (which
/// // would be `BinaryHeap<i32>` in this example).
/// let mut heap = BinaryHeap::new();
///
/// // We can use peek to look at the next item in the heap. In this case,
/// // there's no items in there yet so we get None.
/// assert_eq!(heap.peek(), None);
///
/// // Let's add some scores...
/// heap.push(1);
/// heap.push(5);
/// heap.push(2);
///
/// // Now peek shows the most important item in the heap.
/// assert_eq!(heap.peek(), Some(&5));
///
/// // We can check the length of a heap.
/// assert_eq!(heap.len(), 3);
///
/// // If we instead pop these scores, they should come back in order.
/// assert_eq!(heap.pop(), Some(5));
/// assert_eq!(heap.pop(), Some(2));
/// assert_eq!(heap.pop(), Some(1));
/// assert_eq!(heap.pop(), None);
///
/// ```
///
/// ## Min-heap
///
/// Either `std::cmp::Reverse` or a custom `Ord` implementation can be used to
/// make `BinaryHeap` a min-heap. This makes `heap.pop()` return the smallest
/// value instead of the greatest one.
///
/// ```
/// use tsinghua_ds::BinaryHeap;
/// use std::cmp::Reverse;
///
/// let mut heap = BinaryHeap::new();
///
/// // Wrap values in `Reverse`
/// heap.push(Reverse(1));
/// heap.push(Reverse(5));
/// heap.push(Reverse(2));
///
/// // If we pop these scores now, they should come back in the reverse order.
/// assert_eq!(heap.pop(), Some(Reverse(1)));
/// assert_eq!(heap.pop(), Some(Reverse(2)));
/// assert_eq!(heap.pop(), Some(Reverse(5)));
/// assert_eq!(heap.pop(), None);
/// ```
///
/// # Time complexity
///
/// | [push] | [pop]     | [peek]/[peek\_mut] |
/// |--------|-----------|--------------------|
/// | O(1)~  | *O*(log(*n*)) | *O*(1)               |
///
/// The value for `push` is an expected cost; the method documentation gives a
/// more detailed analysis.
///
/// [push]: BinaryHeap::push
/// [pop]: BinaryHeap::pop
/// [peek]: BinaryHeap::peek
/// [peek\_mut]: BinaryHeap::peek_mut
pub struct BinaryHeap<T> {
    data: Vec<T>,
}

impl<T: Ord + Default> BinaryHeap<T> {
    /// Creates an empty `BinaryHeap` as a max-heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use tsinghua_ds::BinaryHeap;
    /// let mut heap = BinaryHeap::new();
    /// heap.push(4);
    /// ```
    pub fn new() -> BinaryHeap<T> {
        BinaryHeap { data: vec![] }
    }

    /// Creates an empty `BinaryHeap` with a specific capacity.
    /// This preallocates enough memory for `capacity` elements,
    /// so that the `BinaryHeap` does not have to be reallocated
    /// until it contains at least that many values.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use tsinghua_ds::BinaryHeap;
    /// let mut heap = BinaryHeap::with_capacity(10);
    /// heap.push(4);
    /// ```
    pub fn with_capacity(capacity: usize) -> BinaryHeap<T> {
        BinaryHeap {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Returns the greatest item in the binary heap, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use tsinghua_ds::BinaryHeap;
    /// let mut heap = BinaryHeap::new();
    /// assert_eq!(heap.peek(), None);
    ///
    /// heap.push(1);
    /// heap.push(5);
    /// heap.push(2);
    /// assert_eq!(heap.peek(), Some(&5));
    ///
    /// ```
    ///
    /// # Time complexity
    ///
    /// Cost is *O*(1) in the worst case.
    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    /// Pushes an item onto the binary heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use tsinghua_ds::BinaryHeap;
    /// let mut heap = BinaryHeap::new();
    /// heap.push(3);
    /// heap.push(5);
    /// heap.push(1);
    ///
    /// assert_eq!(heap.len(), 3);
    /// assert_eq!(heap.peek(), Some(&5));
    /// ```
    ///
    /// # Time complexity
    ///
    /// The expected cost of `push`, averaged over every possible ordering of
    /// the elements being pushed, and over a sufficiently large number of
    /// pushes, is *O*(1). This is the most meaningful cost metric when pushing
    /// elements that are *not* already in any sorted pattern.
    ///
    /// The time complexity degrades if elements are pushed in predominantly
    /// ascending order. In the worst case, elements are pushed in ascending
    /// sorted order and the amortized cost per push is *O*(log(*n*)) against a heap
    /// containing *n* elements.
    ///
    /// The worst case cost of a *single* call to `push` is *O*(*n*). The worst case
    /// occurs when capacity is exhausted and needs a resize. The resize cost
    /// has been amortized in the previous figures.
    pub fn push(&mut self, item: T) {
        let old_len = self.len();
        self.data.push(item);
        self.sift_up(0, old_len);
    }

    /// sift up
    fn sift_up(&mut self, start: usize, mut pos: usize) {
        let be_replaced = take(&mut self.data[pos]);
        while pos > start {
            let parent = (pos - 1) / 2;
            if self.data[parent] >= be_replaced {
                break;
            }
            self.data[pos] = take(&mut self.data[parent]);
            pos = parent;
        }
        self.data[pos] = be_replaced;
    }

    /// Removes the greatest item from the binary heap and returns it, or `None` if it
    /// is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use tsinghua_ds::BinaryHeap;
    /// let mut heap = BinaryHeap::from(vec![1, 3]);
    ///
    /// assert_eq!(heap.pop(), Some(3));
    /// assert_eq!(heap.pop(), Some(1));
    /// assert_eq!(heap.pop(), None);
    /// ```
    ///
    /// # Time complexity
    ///
    /// The worst case cost of `pop` on a heap containing *n* elements is *O*(log(*n*)).
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop().map(|mut item| {
            if !self.is_empty() {
                swap(&mut item, &mut self.data[0]);
                self.sift_down(0);
            }
            item
        })
    }

    /// Take an element at `pos` and move it all the way down the heap,
    fn sift_down(&mut self, mut pos: usize) {
        let be_replaced = take(&mut self.data[pos]);
        let end = self.len();
        let mut child = 2 * pos + 1;
        while child < end {
            let right = child + 1;
            // compare with the greater of the two children
            if right < end && self.data[child] <= self.data[right] {
                child = right;
            }
            // if we are already in order, stop.
            if be_replaced >= self.data[child] {
                break;
            }

            self.data[pos] = take(&mut self.data[child]);
            pos = child;
            child = 2 * pos + 1;
        }
        self.data[pos] = be_replaced;
    }

    /// Robert Floyd build heap algorithm
    pub fn rebuild(&mut self) {
        let mut n = self.len() / 2;
        while n > 0 {
            n -= 1;
            self.sift_down(n);
        }
    }
    /// Returns the length of the binary heap.
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use tsinghua_ds::BinaryHeap;
    /// let heap = BinaryHeap::from(vec![1, 3]);
    ///
    /// assert_eq!(heap.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the binary heap is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use tsinghua_ds::BinaryHeap;
    /// let mut heap = BinaryHeap::new();
    ///
    /// assert!(heap.is_empty());
    ///
    /// heap.push(3);
    /// heap.push(5);
    /// heap.push(1);
    ///
    /// assert!(!heap.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T: Ord + Default> From<Vec<T>> for BinaryHeap<T> {
    /// Converts a `Vec<T>` into a `BinaryHeap<T>`.
    ///
    /// This conversion happens in-place, and has *O*(*n*) time complexity.
    fn from(vec: Vec<T>) -> BinaryHeap<T> {
        let mut heap = BinaryHeap { data: vec };
        heap.rebuild();
        heap
    }
}

/// Take an element at `pos` and move it all the way down the heap,
fn sift_down<T: Ord + Default>(seq: &mut [T], mut pos: usize, lo: usize, hi: usize) {
    let be_replaced = take(&mut seq[pos]);
    let mut child = 2 * (pos - lo) + lo + 1;
    while child < hi {
        let right = child + 1;
        // compare with the greater of the two children
        if right < hi && seq[child] <= seq[right] {
            child = right;
        }
        // if we are already in order, stop.
        if be_replaced >= seq[child] {
            break;
        }

        seq[pos] = take(&mut seq[child]);
        pos = child;
        child = 2 * (pos - lo) + lo + 1;
    }
    seq[pos] = be_replaced;
}

// use std::fmt::Debug;
/// Heap sort algorithm
/// half-open inclusive
pub fn heap_sort<T: Ord + Default>(seq: &mut [T], lo: usize, mut hi: usize) {
    let mut n = (hi - lo) / 2 + lo;
    // build heap
    while n > lo {
        n -= 1;
        sift_down(seq, n, lo, hi);
    }

    while lo < hi {
        hi -= 1;
        let old_hi = take(&mut seq[hi]);
        let old_lo = take(&mut seq[lo]);
        seq[hi] = old_lo;
        seq[lo] = old_hi;
        sift_down(seq, lo, lo, hi);
    }
}

/// leftist heaps node
#[derive(Debug)]
pub struct TreeNode<T> {
    elem: T,
    npl: i32,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + Default> TreeNode<T> {
    /// Construct new instance
    pub fn new() -> Self {
        Self {
            elem: T::default(),
            npl: -1,
            left: None,
            right: None,
        }
    }

    /// Construct new instance
    pub fn with_elem(elem: T) -> Self {
        Self {
            elem,
            npl: -1,
            left: None,
            right: None,
        }
    }
}

/// Leftist Heap
#[derive(Debug)]
pub struct LeftistHeap<T>(Option<Box<TreeNode<T>>>);

impl<T: Ord + Default> LeftistHeap<T> {
    /// Construct new instance
    pub fn new() -> Self {
        Self(Some(Box::new(TreeNode::new())))
    }
    /// merage algorithm
    pub fn merge(
        mut left_tree: Option<Box<TreeNode<T>>>,
        mut right_tree: Option<Box<TreeNode<T>>>,
    ) -> Option<Box<TreeNode<T>>> {
        if left_tree.is_none() {
            return right_tree;
        }
        if right_tree.is_none() {
            return left_tree;
        }

        if left_tree.as_ref().unwrap().elem < right_tree.as_ref().unwrap().elem {
            swap(&mut left_tree, &mut right_tree);
        }

        if let Some(left_node) = left_tree.as_mut() {
            let right_subtree = left_node.right.take();
            left_node.right = LeftistHeap::merge(right_subtree, right_tree);

            if left_node.left.is_none()
                || left_node.left.as_ref().unwrap().npl < left_node.right.as_ref().unwrap().npl
            {
                swap(&mut left_node.left, &mut left_node.right);
            }

            left_node.npl = if let Some(rchild) = left_node.right.as_ref() {
                rchild.npl + 1
            } else {
                1
            };
        }

        left_tree
    }

    /// insert node
    pub fn insert(&mut self, elem: T) {
        let v = Some(Box::new(TreeNode::with_elem(elem)));
        let old_root = self.0.take();
        self.0 = LeftistHeap::merge(old_root, v);
    }

    /// get node
    pub fn get(&self) -> Option<&T> {
        if let Some(root) = self.0.as_ref() {
            Some(&root.elem)
        } else {
            None
        }
    }

    /// remove node
    pub fn remove(&mut self) -> Option<T> {
        if let Some(mut root) = self.0.take() {
            let (left, right) = (root.left.take(), root.right.take());
            self.0 = LeftistHeap::merge(left, right);

            Some(take(&mut root.elem))
        } else {
            None
        }
    }

    /// clear whole heap and return whole root
    pub fn clear(&mut self) -> Option<Box<TreeNode<T>>> {
        self.0.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_leftistheap() {
        let mut leftheap = LeftistHeap::new();
        let array = [3, 4, 5, 2, 48, 19, 39, 390, 43, 12, 8, 9, 10, 7, 5];
        for &k in array.iter() {
            leftheap.insert(k);
        }
        let b = [2, 3, 4, 5, 5, 7, 8, 9, 10, 12, 19, 39, 43, 48, 390];
        let mut heap = LeftistHeap::new();
        for &k in b.iter() {
            heap.insert(k);
        }
        let _ = LeftistHeap::merge(leftheap.clear(), heap.clear());
    }
    #[test]
    fn test_heap_sort() {
        let mut array = [3, 4, 5, 2, 48, 19, 39, 390, 43, 12, 8, 9, 10, 7, 5];
        let (lo, hi) = (0, array.len());
        heap_sort(&mut array, lo, hi);
        assert_eq!([2, 3, 4, 5, 5, 7, 8, 9, 10, 12, 19, 39, 43, 48, 390], array);

        array = [3, 4, 5, 2, 48, 19, 39, 390, 43, 12, 8, 9, 10, 7, 5];
        let (lo, hi) = (3, array.len() - 5);
        heap_sort(&mut array, lo, hi);
        assert_eq!([3, 4, 5, 2, 12, 19, 39, 43, 48, 390, 8, 9, 10, 7, 5], array);

        array = [3, 4, 5, 2, 48, 19, 39, 390, 43, 12, 8, 9, 10, 7, 5];
        let (lo, hi) = (3, array.len());
        heap_sort(&mut array, lo, hi);
        assert_eq!([3, 4, 5, 2, 5, 7, 8, 9, 10, 12, 19, 39, 43, 48, 390], array);

        let mut array = [3, 4, 5, -3, -10, 2, 48, 19, 39, 390, 43, 12, 8, 9, 10, 7, 5];
        let (lo, hi) = (0, array.len());
        heap_sort(&mut array, lo, hi);
        assert_eq!(
            [-10, -3, 2, 3, 4, 5, 5, 7, 8, 9, 10, 12, 19, 39, 43, 48, 390],
            array
        );

        let mut move_array = [
            format!("{}", 3),
            format!("{}", 4),
            format!("{}", 5),
            format!("{}", 2),
            format!("{}", 48),
            format!("{}", 19),
            format!("{}", 39),
            format!("{}", 390),
            format!("{}", 43),
            format!("{}", 12),
            format!("{}", 8),
            format!("{}", 9),
            format!("{}", 10),
            format!("{}", 7),
            format!("{}", 5),
        ];
        let (lo, hi) = (0, move_array.len());
        heap_sort(&mut move_array, lo, hi);
        assert_eq!(
            [
                format!("{}", 10),
                format!("{}", 12),
                format!("{}", 19),
                format!("{}", 2),
                format!("{}", 3),
                format!("{}", 39),
                format!("{}", 390),
                format!("{}", 4),
                format!("{}", 43),
                format!("{}", 48),
                format!("{}", 5),
                format!("{}", 5),
                format!("{}", 7),
                format!("{}", 8),
                format!("{}", 9),
            ],
            move_array
        );
    }
}
