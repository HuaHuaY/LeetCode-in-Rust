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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(mut n: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
            let node = n.as_mut()?;
            {
                let mut node = node.as_ref().borrow_mut();
                let tmp = node.left.take();
                node.left = dfs(node.right.take());
                node.right = dfs(tmp);
            }
            n
        }
        dfs(root)
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
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let t4 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let t5 = TreeNode::new_with(2, t1, t2);
        let t6 = TreeNode::new_with(7, t3, t4);
        let root = TreeNode::new_with(4, t5, t6);

        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let t4 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t5 = TreeNode::new_with(7, t1, t2);
        let t6 = TreeNode::new_with(2, t3, t4);
        let answer = TreeNode::new_with(4, t5, t6);
        assert_eq!(Solution::invert_tree(root), answer);
    }

    #[test]
    fn test2() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let root = TreeNode::new_with(2, t1, t2);

        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let answer = TreeNode::new_with(2, t1, t2);
        assert_eq!(Solution::invert_tree(root), answer);
    }
}
