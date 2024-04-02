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

use std::collections::HashMap;

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

impl Solution {
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn inner(
            cache: &mut HashMap<i32, Vec<Option<Rc<RefCell<TreeNode>>>>>,
            n: i32,
        ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
            if n & 1 == 0 {
                return vec![];
            }
            if let Some(v) = cache.get(&n) {
                return v.clone();
            }
            let mut result = vec![];
            for i in (1..=n - 2).step_by(2) {
                let left = inner(cache, i);
                let right = inner(cache, n - 1 - i);
                for l in left {
                    for r in &right {
                        result.push(TreeNode::new_with(0, l.clone(), r.clone()));
                    }
                }
            }
            cache.insert(n, result.clone());
            result
        }
        let mut cache = HashMap::with_capacity(n as usize);
        cache.insert(1, vec![Some(Rc::new(RefCell::new(TreeNode::new(0))))]);
        inner(&mut cache, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::all_possible_fbt(3),
            vec![TreeNode::new_with(
                0,
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None
                })))
            )]
        );
    }
}
