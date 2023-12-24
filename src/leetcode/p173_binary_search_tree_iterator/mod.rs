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

#[allow(dead_code)]
struct BSTIterator {
    cur: Option<Rc<RefCell<TreeNode>>>,
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let stack = Vec::new();
        BSTIterator { cur: root, stack }
    }

    fn next(&mut self) -> i32 {
        while self.cur.as_ref().is_some() {
            self.stack.push(Rc::clone(self.cur.as_ref().unwrap()));
            let tmp = self.cur.as_ref().unwrap().borrow_mut().left.take();
            self.cur = tmp;
        }
        self.cur = Some(self.stack.pop().unwrap());
        let result = self.cur.as_ref().unwrap().borrow().val;
        let tmp = self.cur.as_ref().unwrap().borrow_mut().right.take();
        self.cur = tmp;
        result
    }

    fn has_next(&self) -> bool {
        !(self.cur.is_none() && self.stack.is_empty())
    }
}

impl Iterator for BSTIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_next() {
            Some(self.next())
        } else {
            None
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
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(20))));
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let t4 = TreeNode::new_with(15, t1, t2);
        let root = TreeNode::new_with(7, t3, t4);
        let mut obj = BSTIterator::new(root);
        assert_eq!(obj.next(), 3);
        assert_eq!(obj.next(), 7);
        assert!(obj.has_next());
        assert_eq!(obj.next(), 9);
        assert!(obj.has_next());
        assert_eq!(obj.next(), 15);
        assert!(obj.has_next());
        assert_eq!(obj.next(), 20);
        assert!(!obj.has_next());
    }
}
