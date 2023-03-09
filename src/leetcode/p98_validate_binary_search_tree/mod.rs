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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn inorder(root: Option<Rc<RefCell<TreeNode>>>, mut min: i64) -> Option<i64> {
            let root = root.unwrap();
            let mut root = root.borrow_mut();
            if root.left.is_some() {
                min = inorder(root.left.take(), min)?;
            }
            if min >= root.val as i64 {
                return None;
            }
            if root.right.is_some() {
                return inorder(root.right.take(), root.val as i64);
            }
            Some(root.val as i64)
        }
        inorder(root, i64::MIN).is_some()
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
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let root = TreeNode::new_with(2, t1, t2);
        assert!(Solution::is_valid_bst(root));
    }

    #[test]
    fn test2() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let t4 = TreeNode::new_with(4, t2, t3);
        let root = TreeNode::new_with(5, t1, t4);
        assert!(!Solution::is_valid_bst(root));
    }
}
