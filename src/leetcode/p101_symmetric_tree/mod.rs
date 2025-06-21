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
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root = root.unwrap();

        let mut queue = VecDeque::new();
        queue.push_back(root.clone());
        queue.push_back(root);
        while !queue.is_empty() {
            let left = queue.pop_front().unwrap();
            let left = left.borrow();
            let right = queue.pop_front().unwrap();
            let right = right.borrow();
            if left.val != right.val {
                return false;
            }
            if left.left.is_some() && right.right.is_some() {
                queue.push_back(left.left.clone().unwrap());
                queue.push_back(right.right.clone().unwrap());
            } else if left.left.is_some() || right.right.is_some() {
                return false;
            }
            if left.right.is_some() && right.left.is_some() {
                queue.push_back(left.right.clone().unwrap());
                queue.push_back(right.left.clone().unwrap());
            } else if left.right.is_some() || right.left.is_some() {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let t4 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let t5 = TreeNode::new_with(2, t1, t2);
        let t6 = TreeNode::new_with(2, t3, t4);
        let root = TreeNode::new_with(1, t5, t6);
        assert!(Solution::is_symmetric(root));
    }

    #[test]
    fn test2() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let t3 = TreeNode::new_with(2, None, t1);
        let t4 = TreeNode::new_with(2, None, t2);
        let root = TreeNode::new_with(1, t3, t4);
        assert!(!Solution::is_symmetric(root));
    }
}
