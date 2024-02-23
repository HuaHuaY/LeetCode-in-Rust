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
    pub fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
        fn inorder(root: Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
            if let Some(root) = root {
                let mut node = root.borrow_mut();
                inorder(node.left.take(), vec);
                vec.push(node.val);
                inorder(node.right.take(), vec);
            }
        }
        let mut vec = Vec::new();
        inorder(root, &mut vec);
        let mut result = Vec::with_capacity(queries.len());
        for query in queries {
            match vec.binary_search(&query) {
                Ok(_) => result.push(vec![query, query]),
                Err(i) => match i {
                    0 => result.push(vec![-1, vec[i]]),
                    i if i == vec.len() => result.push(vec![vec[i - 1], -1]),
                    _ => result.push(vec![vec[i - 1], vec[i]]),
                },
            }
        }
        result
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
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let t4 = Some(Rc::new(RefCell::new(TreeNode::new(14))));
        let t5 = TreeNode::new_with(2, t1, t2);
        let t6 = TreeNode::new_with(15, t4, None);
        let t7 = TreeNode::new_with(13, t3, t6);
        let root = TreeNode::new_with(6, t5, t7);
        assert_eq!(
            Solution::closest_nodes(root, [2, 5, 16].to_vec()),
            [[2, 2], [4, 6], [15, -1]]
        );
    }

    #[test]
    fn test2() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let root = TreeNode::new_with(4, None, t1);
        assert_eq!(Solution::closest_nodes(root, [3].to_vec()), [[-1, 4]]);
    }
}
