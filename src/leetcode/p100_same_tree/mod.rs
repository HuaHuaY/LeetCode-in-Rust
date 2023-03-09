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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn preorder(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<Option<i32>>) {
            if let Some(root) = root {
                let root = root.borrow_mut();
                result.push(Some(root.val));
                preorder(&root.left, result);
                preorder(&root.right, result);
            } else {
                result.push(None);
            }
        }
        fn inorder(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<Option<i32>>) {
            if let Some(root) = root {
                let root = root.borrow_mut();
                inorder(&root.left, result);
                result.push(Some(root.val));
                inorder(&root.right, result);
            } else {
                result.push(None);
            }
        }
        let mut res1_preorder = vec![];
        preorder(&p, &mut res1_preorder);
        let mut res2_preorder = vec![];
        preorder(&q, &mut res2_preorder);
        let mut res1_inorder = vec![];
        inorder(&p, &mut res1_inorder);
        let mut res2_inorder = vec![];
        inorder(&q, &mut res2_inorder);
        res1_preorder == res2_preorder && res1_inorder == res2_inorder
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
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let root1 = TreeNode::new_with(1, t1, t2);
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let t4 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let root2 = TreeNode::new_with(1, t3, t4);
        assert!(Solution::is_same_tree(root1, root2));
    }

    #[test]
    fn test2() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let root1 = TreeNode::new_with(1, t1, None);
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let root2 = TreeNode::new_with(1, None, t2);
        assert!(!Solution::is_same_tree(root1, root2));
    }

    #[test]
    fn test3() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let root1 = TreeNode::new_with(1, t1, t2);
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t4 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let root2 = TreeNode::new_with(1, t3, t4);
        assert!(!Solution::is_same_tree(root1, root2));
    }

    #[test]
    fn test4() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let root1 = TreeNode::new_with(1, t1, None);
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let root2 = TreeNode::new_with(1, None, t2);
        assert!(!Solution::is_same_tree(root1, root2));
    }
}
