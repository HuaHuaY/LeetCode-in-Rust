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
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        fn preorder(root: Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i64>, index: usize) {
            if let Some(node) = root {
                let mut node = node.borrow_mut();
                if vec.len() == index {
                    vec.push(0);
                }
                vec[index] += node.val as i64;
                preorder(node.left.take(), vec, index + 1);
                preorder(node.right.take(), vec, index + 1);
            }
        }
        let mut vec = Vec::new();
        preorder(root, &mut vec, 0);

        let len = vec.len();
        if len < k as usize {
            return -1;
        }

        let (_, result, _) = vec.select_nth_unstable(len - k as usize);
        *result
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
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let t3 = TreeNode::new_with(2, t1, t2);
        let t4 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t5 = TreeNode::new_with(8, t3, t4);
        let t6 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let t7 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let t8 = TreeNode::new_with(9, t6, t7);
        let root = TreeNode::new_with(5, t5, t8);
        assert_eq!(Solution::kth_largest_level_sum(root, 2), 13);
    }

    #[test]
    fn test2() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let t2 = TreeNode::new_with(2, t1, None);
        let root = TreeNode::new_with(1, t2, None);
        assert_eq!(Solution::kth_largest_level_sum(root, 1), 3);
    }
}
