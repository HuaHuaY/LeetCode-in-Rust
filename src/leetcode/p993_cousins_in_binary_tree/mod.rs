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
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let mut vec1 = Vec::new();
        let mut vec2 = Vec::new();
        vec1.push(root.as_ref().unwrap().clone());
        while !vec1.is_empty() {
            let mut i = 0;
            for node in vec1.iter() {
                let node = node.borrow();
                let mut j = 0;
                if let Some(left) = node.left.as_ref() {
                    if left.borrow().val == x || left.borrow().val == y {
                        j += 1;
                    }
                    vec2.push(left.clone());
                }
                if let Some(right) = node.right.as_ref() {
                    if right.borrow().val == x || right.borrow().val == y {
                        j += 1;
                    }
                    vec2.push(right.clone());
                }
                match j {
                    0 => {}
                    1 => i += 1,
                    2 => return false,
                    _ => unreachable!(),
                }
            }
            match i {
                0 => {}
                1 => return false,
                2 => return true,
                _ => unreachable!(),
            }
            std::mem::swap(&mut vec1, &mut vec2);
            vec2.clear();
        }
        unreachable!()
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
        let t2 = TreeNode::new_with(2, t1, None);
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let root = TreeNode::new_with(1, t2, t3);
        assert!(!Solution::is_cousins(root, 4, 3));
    }

    #[test]
    fn test2() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let t2 = TreeNode::new_with(2, t1, None);
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let t4 = TreeNode::new_with(3, None, t3);
        let root = TreeNode::new_with(1, t2, t4);
        assert!(Solution::is_cousins(root, 5, 4));
    }

    #[test]
    fn test3() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let t2 = TreeNode::new_with(2, None, t1);
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let root = TreeNode::new_with(1, t2, t3);
        assert!(!Solution::is_cousins(root, 2, 3));
    }
}
