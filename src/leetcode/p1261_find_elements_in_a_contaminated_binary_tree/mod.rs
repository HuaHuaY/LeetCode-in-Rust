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

struct FindElements {
    root: Option<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self { root }
    }

    fn find(&self, mut target: i32) -> bool {
        target += 1;

        let mut cur = self.root.clone();
        for i in (0..31 - target.leading_zeros()).rev() {
            cur = if (target >> i) & 1 == 0 {
                cur.unwrap().borrow().left.clone()
            } else {
                cur.unwrap().borrow().right.clone()
            };
            if cur.is_none() {
                return false;
            }
        }
        true
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */
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
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(-1))));
        let root = TreeNode::new_with(-1, None, t1);
        let obj = FindElements::new(root);
        assert!(!obj.find(1));
        assert!(obj.find(2));
    }

    #[test]
    fn test2() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(-1))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(-1))));
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(-1))));
        let t4 = TreeNode::new_with(-1, t1, t2);
        let root = TreeNode::new_with(-1, t4, t3);
        let obj = FindElements::new(root);
        assert!(obj.find(1));
        assert!(obj.find(3));
        assert!(!obj.find(5));
    }

    #[test]
    fn test3() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(-1))));
        let t2 = TreeNode::new_with(-1, t1, None);
        let t3 = TreeNode::new_with(-1, t2, None);
        let root = TreeNode::new_with(-1, None, t3);
        let obj = FindElements::new(root);
        assert!(obj.find(2));
        assert!(!obj.find(3));
        assert!(!obj.find(4));
        assert!(obj.find(5));
    }
}
