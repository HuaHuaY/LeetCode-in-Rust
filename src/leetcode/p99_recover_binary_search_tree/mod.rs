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
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut root = root.clone();
        let mut x: Option<Rc<RefCell<TreeNode>>> = None;
        let mut y: Option<Rc<RefCell<TreeNode>>> = None;
        let mut pred: Option<Rc<RefCell<TreeNode>>> = None;
        while let Some(root_inner) = root {
            let r = root_inner.borrow();
            if let Some(mut predecessor) = r.left.clone() {
                while let Some(right) = {
                    let borrow = predecessor.borrow();
                    let t = borrow.right.clone();
                    drop(borrow);
                    t
                } {
                    if right.borrow().val == r.val {
                        break;
                    }
                    predecessor = right;
                }
                let mut predecessor_ref_mut = predecessor.borrow_mut();
                if predecessor_ref_mut.right.is_none() {
                    predecessor_ref_mut.right = Some(root_inner.clone());
                    root = r.left.clone();
                } else {
                    if predecessor_ref_mut.val > r.val {
                        y = Some(root_inner.clone());
                        if x.is_none() {
                            x = Some(predecessor.clone());
                        }
                    }
                    pred = Some(root_inner.clone());
                    predecessor_ref_mut.right = None;
                    root = r.right.clone();
                }
            } else {
                if let Some(p) = pred {
                    if p.borrow().val > r.val {
                        y = Some(root_inner.clone());
                        if x.is_none() {
                            x = Some(p);
                        }
                    }
                }
                pred = Some(root_inner.clone());
                root = r.right.clone();
            }
        }
        let x = x.unwrap();
        let mut x = x.borrow_mut();
        let y = y.unwrap();
        let mut y = y.borrow_mut();
        std::mem::swap(&mut x.val, &mut y.val);
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
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let t2 = TreeNode::new_with(3, None, t1);
        let mut root = TreeNode::new_with(1, t2, None);
        Solution::recover_tree(&mut root);
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let t4 = TreeNode::new_with(1, None, t3);
        let ans = TreeNode::new_with(3, t4, None);
        assert_eq!(root, ans);
    }

    #[test]
    fn test2() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let t3 = TreeNode::new_with(4, t2, None);
        let mut root = TreeNode::new_with(3, t1, t3);
        Solution::recover_tree(&mut root);
        let t4 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t5 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let t6 = TreeNode::new_with(4, t5, None);
        let ans = TreeNode::new_with(2, t4, t6);
        assert_eq!(root, ans);
    }
}
