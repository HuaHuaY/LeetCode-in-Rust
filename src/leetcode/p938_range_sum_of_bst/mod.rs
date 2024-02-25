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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        if let Some(root) = root {
            let mut root = root.borrow_mut();
            let v = root.val;
            match v {
                v if v < low => {
                    return Self::range_sum_bst(root.right.take(), low, high);
                }
                v if v > high => {
                    return Self::range_sum_bst(root.left.take(), low, high);
                }
                _ => {
                    return v
                        + Self::range_sum_bst(root.left.take(), low, high)
                        + Self::range_sum_bst(root.right.take(), low, high);
                }
            }
        }
        0
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
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let t3 = TreeNode::new_with(5, t1, t2);
        let t4 = Some(Rc::new(RefCell::new(TreeNode::new(18))));
        let t5 = TreeNode::new_with(15, None, t4);
        let root = TreeNode::new_with(10, t3, t5);
        assert_eq!(Solution::range_sum_bst(root, 7, 15), 32);
    }

    #[test]
    fn test2() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let t3 = TreeNode::new_with(3, t1, None);
        let t4 = TreeNode::new_with(7, t2, None);
        let t5 = TreeNode::new_with(5, t3, t4);
        let t6 = Some(Rc::new(RefCell::new(TreeNode::new(13))));
        let t7 = Some(Rc::new(RefCell::new(TreeNode::new(18))));
        let t8 = TreeNode::new_with(15, t6, t7);
        let root = TreeNode::new_with(10, t5, t8);
        assert_eq!(Solution::range_sum_bst(root, 6, 10), 23);
    }
}
