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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn preorder(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
            if let Some(root) = root {
                let mut root = root.borrow_mut();
                result.push(root.val);
                preorder(root.left.take(), result);
                preorder(root.right.take(), result);
            }
        }
        let mut result = vec![];
        preorder(root, &mut result);
        result
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
        let t2 = TreeNode::new_with(2, t1, None);
        let root = TreeNode::new_with(1, None, t2);
        assert_eq!(Solution::preorder_traversal(root), [1, 2, 3]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::preorder_traversal(None), [] as [i32; 0]);
    }

    #[test]
    fn test3() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(Solution::preorder_traversal(root), [1]);
    }
}
