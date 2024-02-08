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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || root == p || root == q {
            return root;
        }
        let r = root.as_ref().unwrap().clone();
        let rb = r.borrow();
        let left = Solution::lowest_common_ancestor(rb.left.clone(), p.clone(), q.clone());
        let right = Solution::lowest_common_ancestor(rb.right.clone(), p.clone(), q.clone());
        match (left.is_some(), right.is_some()) {
            (true, true) => root,
            (true, false) => left,
            (false, true) => right,
            (false, false) => None,
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
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let t4 = TreeNode::new_with(2, t2, t3);
        let t5 = TreeNode::new_with(5, t1, t4);
        let t6 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let t7 = Some(Rc::new(RefCell::new(TreeNode::new(8))));
        let t8 = TreeNode::new_with(1, t6, t7);
        let root = TreeNode::new_with(3, t5.clone(), t8.clone());
        assert_eq!(Solution::lowest_common_ancestor(root.clone(), t5, t8), root);
    }

    #[test]
    fn test2() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let t4 = TreeNode::new_with(2, t2, t3.clone());
        let t5 = TreeNode::new_with(5, t1, t4);
        let t6 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let t7 = Some(Rc::new(RefCell::new(TreeNode::new(8))));
        let t8 = TreeNode::new_with(1, t6, t7);
        let root = TreeNode::new_with(3, t5.clone(), t8);
        assert_eq!(Solution::lowest_common_ancestor(root, t5.clone(), t3), t5);
    }

    #[test]
    fn test3() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let root = TreeNode::new_with(1, t1.clone(), None);
        assert_eq!(
            Solution::lowest_common_ancestor(root.clone(), root.clone(), t1),
            root
        );
    }
}
