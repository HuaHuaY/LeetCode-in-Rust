pub struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

use std::collections::{BTreeMap, BinaryHeap, VecDeque};

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut map = BTreeMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((root, 0, 0));
        while !queue.is_empty() {
            let (node, x, y) = queue.pop_front().unwrap();
            if let Some(node) = node {
                let mut node = node.borrow_mut();
                map.entry(y)
                    .or_insert(BinaryHeap::new())
                    .push((x, node.val));
                queue.push_back((node.left.take(), x + 1, y - 1));
                queue.push_back((node.right.take(), x + 1, y + 1));
            }
        }
        map.into_values()
            .map(|h| h.into_sorted_vec().into_iter().map(|(_, v)| v).collect())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    impl TreeNode {
        #[inline]
        fn new_with(
            val: i32,
            left: Option<Rc<RefCell<TreeNode>>>,
            right: Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        }
    }

    #[test]
    fn test1() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let t4 = TreeNode::new_with(20, t2, t3);
        let root = TreeNode::new_with(3, t1, t4);
        assert_eq!(
            Solution::vertical_traversal(root),
            vec_vec![[9], [3, 15], [20], [7]]
        );
    }

    #[test]
    fn test2() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let t3 = TreeNode::new_with(2, t1, t2);
        let t4 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let t5 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let t6 = TreeNode::new_with(3, t4, t5);
        let root = TreeNode::new_with(1, t3, t6);
        assert_eq!(
            Solution::vertical_traversal(root),
            vec_vec![[4], [2], [1, 5, 6], [3], [7]]
        );
    }

    #[test]
    fn test3() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let t3 = TreeNode::new_with(2, t1, t2);
        let t4 = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let t5 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let t6 = TreeNode::new_with(3, t4, t5);
        let root = TreeNode::new_with(1, t3, t6);
        assert_eq!(
            Solution::vertical_traversal(root),
            vec_vec![[4], [2], [1, 5, 6], [3], [7]]
        );
    }
}
