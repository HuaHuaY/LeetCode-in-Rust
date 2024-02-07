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
    pub fn replace_value_in_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut vec1 = Vec::new();
        let mut vec2 = Vec::new();
        let mut vec3 = Vec::new();
        vec1.push(root.as_ref().unwrap().clone());
        while !vec1.is_empty() {
            let mut sum = 0;
            for node in vec1.iter() {
                let mut t = 0;
                if let Some(left) = &node.borrow().left {
                    t += left.borrow().val;
                    vec2.push(left.clone());
                }
                if let Some(right) = &node.borrow().right {
                    t += right.borrow().val;
                    vec2.push(right.clone());
                }
                sum += t;
                vec3.push(t);
            }
            for (node, &t) in vec1.iter().zip(&vec3) {
                if let Some(left) = &node.borrow().left {
                    left.borrow_mut().val = sum - t;
                }
                if let Some(right) = &node.borrow().right {
                    right.borrow_mut().val = sum - t;
                }
            }
            std::mem::swap(&mut vec1, &mut vec2);
            vec2.clear();
            vec3.clear();
        }
        root.as_ref().unwrap().borrow_mut().val = 0;
        root
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
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(10))));
        let t3 = TreeNode::new_with(4, t1, t2);
        let t4 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let t5 = TreeNode::new_with(9, None, t4);
        let root1 = TreeNode::new_with(5, t3, t5);

        let t6 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let t7 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let t8 = TreeNode::new_with(0, t6, t7);
        let t9 = Some(Rc::new(RefCell::new(TreeNode::new(11))));
        let t10 = TreeNode::new_with(0, None, t9);
        let root2 = TreeNode::new_with(0, t8, t10);

        assert_eq!(Solution::replace_value_in_tree(root1), root2);
    }

    #[test]
    fn test2() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let root1 = TreeNode::new_with(3, t1, t2);

        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let t4 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let root2 = TreeNode::new_with(0, t3, t4);

        assert_eq!(Solution::replace_value_in_tree(root1), root2);
    }
}
