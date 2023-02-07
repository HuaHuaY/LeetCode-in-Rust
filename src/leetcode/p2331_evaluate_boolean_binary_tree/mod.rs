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
impl Solution {
    pub fn evaluate_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut root = root.as_mut().unwrap().borrow_mut();
        match root.val {
            0 => false,
            1 => true,
            2 => {
                Solution::evaluate_tree(root.left.take())
                    || Solution::evaluate_tree(root.right.take())
            }
            3 => {
                Solution::evaluate_tree(root.left.take())
                    && Solution::evaluate_tree(root.right.take())
            }
            _ => unreachable!(),
        }
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
        let t1 = None;
        let t2 = None;
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let t4 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t5 = TreeNode::new_with(1, t1, t2);
        let t6 = TreeNode::new_with(3, t3, t4);
        let root = TreeNode::new_with(2, t5, t6);
        assert!(Solution::evaluate_tree(root));
    }

    #[test]
    fn test2() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        assert!(!Solution::evaluate_tree(root));
    }
}
