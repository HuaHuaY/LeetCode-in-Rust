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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn inner(
            root: Option<Rc<RefCell<TreeNode>>>,
            p: i32,
            q: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            let root = root.unwrap();
            let mut node = root.borrow_mut();
            let v = node.val;
            match v {
                v if v > q => inner(node.left.take(), p, q),
                v if v < p => inner(node.right.take(), p, q),
                _ => {
                    drop(node);
                    Some(root)
                }
            }
        }
        let mut p = p.as_ref().unwrap().borrow().val;
        let mut q = q.as_ref().unwrap().borrow().val;
        if p > q {
            (p, q) = (q, p)
        }
        inner(root, p, q)
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
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let t4 = TreeNode::new_with(4, t2, t3);
        let t5 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let t6 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let t7 = TreeNode::new_with(2, t1, t4);
        let t8 = TreeNode::new_with(8, t5, t6);
        let root = TreeNode::new_with(6, t7.clone(), t8.clone());
        assert_eq!(Solution::lowest_common_ancestor(root.clone(), t7, t8), root);
    }

    #[test]
    fn test2() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let t4 = TreeNode::new_with(4, t2, t3);
        let t5 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let t6 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let t7 = TreeNode::new_with(2, t1, t4.clone());
        let t8 = TreeNode::new_with(8, t5, t6);
        let root = TreeNode::new_with(6, t7.clone(), t8);
        assert_eq!(Solution::lowest_common_ancestor(root, t7.clone(), t4), t7);
    }

    #[test]
    fn test3() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let root = TreeNode::new_with(2, t1.clone(), None);
        assert_eq!(
            Solution::lowest_common_ancestor(root.clone(), root.clone(), t1),
            root.clone()
        );
    }
}
