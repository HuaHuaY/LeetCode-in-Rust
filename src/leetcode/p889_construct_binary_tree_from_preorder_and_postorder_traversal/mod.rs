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
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_tree_inner(preorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            match preorder.len() {
                0 => unreachable!(),
                1 => Some(Rc::new(RefCell::new(TreeNode::new(preorder[0])))),
                _ => {
                    let root = preorder[0];
                    let left_root = preorder[1];
                    let index = postorder.iter().position(|e| *e == left_root).unwrap();
                    let left = build_tree_inner(&preorder[1..=index + 1], &postorder[..=index]);
                    let mut right = None;
                    let len = postorder.len();
                    if index < len - 2 {
                        right = build_tree_inner(
                            &preorder[index + 2..],
                            &postorder[index + 1..len - 1],
                        );
                    }
                    TreeNode::new_with(root, left, right)
                }
            }
        }
        build_tree_inner(&preorder, &postorder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let t4 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let t5 = TreeNode::new_with(2, t1, t2);
        let t6 = TreeNode::new_with(3, t3, t4);
        let root = TreeNode::new_with(1, t5, t6);
        assert_eq!(
            Solution::construct_from_pre_post(
                [1, 2, 4, 5, 3, 6, 7].to_vec(),
                [4, 5, 2, 6, 7, 3, 1].to_vec()
            ),
            root
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::construct_from_pre_post([1].to_vec(), [1].to_vec()),
            Some(Rc::new(RefCell::new(TreeNode::new(1))))
        );
    }
}
