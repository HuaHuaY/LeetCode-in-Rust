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
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Rc<RefCell<TreeNode>>, max_v: i32, min_v: i32) -> i32 {
            let b = node.borrow();
            let mut max = (max_v - b.val).max(b.val - min_v);
            for son in [b.left.as_ref(), b.right.as_ref()].iter().flatten() {
                max = max.max(dfs(Rc::clone(son), max_v.max(b.val), min_v.min(b.val)));
            }
            max
        }
        dfs(root.unwrap(), 0, 100000)
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
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t4 = TreeNode::new_with(6, t1, t2);
        let t5 = TreeNode::new_with(3, t3, t4);
        let t6 = Some(Rc::new(RefCell::new(TreeNode::new(13))));
        let t7 = TreeNode::new_with(14, t6, None);
        let t8 = TreeNode::new_with(10, None, t7);
        let root = TreeNode::new_with(8, t5, t8);
        assert_eq!(Solution::max_ancestor_diff(root), 7);
    }

    #[test]
    fn test2() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let t2 = TreeNode::new_with(0, t1, None);
        let t3 = TreeNode::new_with(2, None, t2);
        let root = TreeNode::new_with(1, None, t3);
        assert_eq!(Solution::max_ancestor_diff(root), 3);
    }
}
