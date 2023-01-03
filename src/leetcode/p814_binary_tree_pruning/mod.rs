use std::cell::RefCell;
use std::rc::Rc;

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

pub struct Solution;

impl Solution {
    fn traverse(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut tree_node = root.as_ref().unwrap().borrow_mut();
        let l_value = Solution::traverse(tree_node.left.clone());
        if l_value == 0 {
            tree_node.left = None;
        }
        let r_value = Solution::traverse(tree_node.right.clone());
        if r_value == 0 {
            tree_node.right = None;
        }
        l_value + tree_node.val + r_value
    }

    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if Solution::traverse(root.clone()) == 0 {
            None
        } else {
            root
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
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t3 = TreeNode::new_with(0, t1, t2);
        let root = TreeNode::new_with(1, None, t3);

        let t4 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t5 = TreeNode::new_with(0, None, t4);
        let answer = TreeNode::new_with(1, None, t5);

        assert_eq!(Solution::prune_tree(root), answer);
    }

    #[test]
    fn test2() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let t4 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t5 = TreeNode::new_with(0, t1, t2);
        let t6 = TreeNode::new_with(1, t3, t4);
        let root = TreeNode::new_with(1, t5, t6);

        let t7 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t8 = TreeNode::new_with(1, None, t7);
        let answer = TreeNode::new_with(1, None, t8);

        assert_eq!(Solution::prune_tree(root), answer);
    }

    #[test]
    fn test3() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let t4 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t5 = TreeNode::new_with(1, t1, None);
        let t6 = TreeNode::new_with(1, t5, t2);
        let t7 = TreeNode::new_with(0, t3, t4);
        let root = TreeNode::new_with(1, t6, t7);

        let t8 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t9 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t10 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t11 = TreeNode::new_with(1, t8, t9);
        let t12 = TreeNode::new_with(0, None, t10);
        let answer = TreeNode::new_with(1, t11, t12);

        assert_eq!(Solution::prune_tree(root), answer);
    }
}
