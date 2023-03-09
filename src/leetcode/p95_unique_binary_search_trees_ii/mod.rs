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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn generate_trees_inner(left: i32, right: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
            let mut result = Vec::new();
            for i in left..=right {
                let left = if i > left {
                    generate_trees_inner(left, i - 1)
                } else {
                    vec![None]
                };

                let right = if i < right {
                    generate_trees_inner(i + 1, right)
                } else {
                    vec![None]
                };

                for l in &left {
                    for r in &right {
                        let mut root = TreeNode::new(i);
                        root.left = l.clone();
                        root.right = r.clone();
                        result.push(Some(Rc::new(RefCell::new(root))));
                    }
                }
            }
            result
        }
        generate_trees_inner(1, n)
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
        let t2 = TreeNode::new_with(3, t1, None);
        let tree1 = TreeNode::new_with(1, None, t2);
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let t4 = TreeNode::new_with(2, None, t3);
        let tree2 = TreeNode::new_with(1, None, t4);
        let t5 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t6 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let tree3 = TreeNode::new_with(2, t5, t6);
        let t7 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t8 = TreeNode::new_with(2, t7, None);
        let tree4 = TreeNode::new_with(3, t8, None);
        let t9 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let t10 = TreeNode::new_with(1, None, t9);
        let tree5 = TreeNode::new_with(3, t10, None);
        let vec1 = Solution::generate_trees(3);
        let vec2 = [tree1, tree2, tree3, tree4, tree5].to_vec();
        assert_eq!(vec1.len(), vec2.len());
        assert!(vec2.into_iter().all(|t| vec1.contains(&t)));
    }

    #[test]
    fn test2() {
        let tree1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let vec1 = Solution::generate_trees(1);
        let vec2 = [tree1].to_vec();
        assert_eq!(vec1.len(), vec2.len());
        assert!(vec2.into_iter().all(|t| vec1.contains(&t)));
    }
}
