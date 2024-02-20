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

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_tree_inner(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            let len = postorder.len();
            let root = postorder[len - 1];
            let index = inorder.iter().position(|&x| x == root).unwrap();
            let mut left = None;
            let mut right = None;
            if index >= 1 {
                left = build_tree_inner(&inorder[..index], &postorder[..index]);
            }
            if index + 1 < len {
                right = build_tree_inner(&inorder[index + 1..], &postorder[index..len - 1]);
            }
            TreeNode::new_with(root, left, right)
        }
        build_tree_inner(&inorder, &postorder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let t4 = TreeNode::new_with(20, t2, t3);
        let root = TreeNode::new_with(3, t1, t4);
        assert_eq!(
            Solution::build_tree([9, 3, 15, 20, 7].to_vec(), [9, 15, 7, 20, 3].to_vec()),
            root
        );
    }

    #[test]
    fn test2() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(-1))));
        assert_eq!(Solution::build_tree([-1].to_vec(), [-1].to_vec()), root);
    }
}
