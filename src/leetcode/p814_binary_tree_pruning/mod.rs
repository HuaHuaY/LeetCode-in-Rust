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

pub struct Solution {}
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

    #[test]
    fn test1() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.as_ref().unwrap().borrow_mut().left = None;
        root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        root.as_ref()
            .unwrap()
            .borrow_mut()
            .right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        root.as_ref()
            .unwrap()
            .borrow_mut()
            .right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let result = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        result.as_ref().unwrap().borrow_mut().left = None;
        result.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        result
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .left = None;
        result
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(Solution::prune_tree(root), result);
    }

    #[test]
    fn test2() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.as_ref()
            .unwrap()
            .borrow_mut()
            .left
            .as_ref()
            .unwrap()
            .borrow_mut()
            .left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        root.as_ref()
            .unwrap()
            .borrow_mut()
            .left
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        root.as_ref()
            .unwrap()
            .borrow_mut()
            .right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        root.as_ref()
            .unwrap()
            .borrow_mut()
            .right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let result = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        result.as_ref().unwrap().borrow_mut().left = None;
        result.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        result
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .left = None;
        result
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(Solution::prune_tree(root), result);
    }

    #[test]
    fn test3() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        root.as_ref()
            .unwrap()
            .borrow_mut()
            .left
            .as_ref()
            .unwrap()
            .borrow_mut()
            .left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.as_ref()
            .unwrap()
            .borrow_mut()
            .left
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.as_ref()
            .unwrap()
            .borrow_mut()
            .right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        root.as_ref()
            .unwrap()
            .borrow_mut()
            .right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.as_ref()
            .unwrap()
            .borrow_mut()
            .left
            .as_ref()
            .unwrap()
            .borrow_mut()
            .left
            .as_ref()
            .unwrap()
            .borrow_mut()
            .left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let result = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        result.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        result.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        result
            .as_ref()
            .unwrap()
            .borrow_mut()
            .left
            .as_ref()
            .unwrap()
            .borrow_mut()
            .left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        result
            .as_ref()
            .unwrap()
            .borrow_mut()
            .left
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        result
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .left = None;
        result
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(Solution::prune_tree(root), result);
    }
}
