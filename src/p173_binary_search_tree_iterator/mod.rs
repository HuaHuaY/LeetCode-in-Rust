use std::cell::RefCell;
use std::rc::Rc;

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

pub struct BSTIterator {
    cur: Option<Rc<RefCell<TreeNode>>>,
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let stack = Vec::new();
        BSTIterator { cur: root, stack }
    }

    pub fn next(&mut self) -> i32 {
        while let Some(_) = self.cur.as_ref() {
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

    pub fn has_next(&self) -> bool {
        !(self.cur == None && self.stack.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(20)))),
            }))),
        })));
        let mut obj = BSTIterator::new(root);
        assert_eq!(obj.next(), 3);
        assert_eq!(obj.next(), 7);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 9);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 15);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 20);
        assert_eq!(obj.has_next(), false);
    }
}
