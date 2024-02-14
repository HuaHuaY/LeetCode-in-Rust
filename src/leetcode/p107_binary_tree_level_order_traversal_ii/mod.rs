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
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn foo(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<Vec<i32>>, high: usize) {
            if let Some(node) = root {
                if result.len() <= high {
                    result.push(Vec::new());
                }
                let mut node = node.borrow_mut();
                result[high].push(node.val);
                foo(node.left.take(), result, high + 1);
                foo(node.right.take(), result, high + 1);
            }
        }
        let mut result = Vec::new();
        foo(root, &mut result, 0);
        result.reverse();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

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
        assert_eq!(
            Solution::level_order_bottom(root),
            vec_vec![[15, 7], [9, 20], [3]]
        );
    }

    #[test]
    fn test2() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(Solution::level_order_bottom(root), vec_vec![[1]]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::level_order_bottom(None), [] as [Vec<i32>; 0]);
    }
}
