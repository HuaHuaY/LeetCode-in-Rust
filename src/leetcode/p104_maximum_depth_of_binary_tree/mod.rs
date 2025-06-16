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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
            if let Some(n) = node {
                let left_depth = dfs(n.borrow().left.clone(), depth + 1);
                let right_depth = dfs(n.borrow().right.clone(), depth + 1);
                return left_depth.max(right_depth);
            }
            depth
        }
        dfs(root, 0)
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
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let t4 = TreeNode::new_with(20, t2, t3);
        let root = TreeNode::new_with(3, t1, t4);
        assert_eq!(Solution::max_depth(root), 3);
    }

    #[test]
    fn test2() {
        let t1 = None;
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let root = TreeNode::new_with(1, t1, t2);
        assert_eq!(Solution::max_depth(root), 2);
    }
}
