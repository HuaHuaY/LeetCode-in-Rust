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
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
        fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> usize {
            if root.is_none() {
                return 0;
            }

            let mut node = root.as_ref().unwrap().borrow_mut();
            1 + count_nodes(node.left.take()) + count_nodes(node.right.take())
        }

        let mut first_node = {
            let mut first_node = None;
            let mut queue = VecDeque::new();
            queue.push_back(root);
            while !queue.is_empty() {
                let mut top = queue.pop_front().unwrap();
                if top.as_ref().unwrap().borrow().val == x {
                    first_node = top;
                    break;
                }
                let mut tree_node = top.as_mut().unwrap().borrow_mut();
                let left = tree_node.left.take();
                if left.is_some() {
                    queue.push_back(left);
                }
                let right = tree_node.right.take();
                if right.is_some() {
                    queue.push_back(right);
                }
            }
            first_node
        };
        let mut first_tree_node = first_node.as_mut().unwrap().borrow_mut();

        let left_son_num = count_nodes(first_tree_node.left.take());
        let right_son_num = count_nodes(first_tree_node.right.take());
        let else_num = n as usize - left_son_num - right_son_num - 1;
        left_son_num > n as usize / 2 || right_son_num > n as usize / 2 || else_num > n as usize / 2
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
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(8))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(10))));
        let t4 = Some(Rc::new(RefCell::new(TreeNode::new(11))));
        let t5 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let t6 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let t7 = TreeNode::new_with(4, t1, t2);
        let t8 = TreeNode::new_with(5, t3, t4);
        let t9 = TreeNode::new_with(2, t7, t8);
        let t10 = TreeNode::new_with(3, t5, t6);
        let root = TreeNode::new_with(1, t9, t10);
        assert!(Solution::btree_game_winning_move(root, 11, 3));
    }

    #[test]
    fn test2() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let root = TreeNode::new_with(1, t1, t2);
        assert!(!Solution::btree_game_winning_move(root, 3, 1));
    }
}
