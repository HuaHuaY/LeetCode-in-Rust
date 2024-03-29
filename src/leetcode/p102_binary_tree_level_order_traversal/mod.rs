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

use std::collections::VecDeque;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }

        let mut queue1 = VecDeque::new();
        let mut queue2 = VecDeque::new();
        queue1.push_back(root);
        let mut result = vec![];
        while !queue1.is_empty() {
            let mut r = vec![];
            while let Some(node) = queue1.pop_front() {
                let node = node.unwrap();
                let mut node = node.borrow_mut();
                r.push(node.val);
                if node.left.is_some() {
                    queue2.push_back(node.left.take());
                }
                if node.right.is_some() {
                    queue2.push_back(node.right.take());
                }
            }
            result.push(r);
            std::mem::swap(&mut queue1, &mut queue2);
        }
        result
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
        assert_eq!(Solution::level_order(root), vec_vec![[3], [9, 20], [15, 7]]);
    }

    #[test]
    fn test2() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(Solution::level_order(root), vec_vec![[1]]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::level_order(None), [] as [Vec<i32>; 0]);
    }
}
