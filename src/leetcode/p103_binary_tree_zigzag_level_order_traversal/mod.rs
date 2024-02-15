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

use std::collections::VecDeque;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut queue1 = VecDeque::new();
        let mut queue2 = VecDeque::new();
        queue1.push_front(root);
        let mut rev = true;
        while !queue1.is_empty() {
            let mut level = Vec::new();
            while let Some(node) = queue1.pop_front() {
                if let Some(node) = node {
                    let node = node.borrow();
                    level.push(node.val);
                    if rev {
                        queue2.push_front(node.left.clone());
                        queue2.push_front(node.right.clone());
                    } else {
                        queue2.push_front(node.right.clone());
                        queue2.push_front(node.left.clone());
                    }
                }
            }
            if !level.is_empty() {
                result.push(level);
            }
            std::mem::swap(&mut queue1, &mut queue2);
            rev = !rev;
        }
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
            Solution::zigzag_level_order(root),
            vec_vec![[3], [20, 9], [15, 7]]
        );
    }

    #[test]
    fn test2() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(Solution::zigzag_level_order(root), vec_vec![[1]]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::zigzag_level_order(None), [] as [Vec<i32>; 0]);
    }
}
