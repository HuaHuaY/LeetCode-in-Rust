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
    fn dfs(sums: &mut Vec<i32>, node: Option<Rc<RefCell<TreeNode>>>, level: usize) {
        if node.is_none() {
            return;
        }

        let node = node.unwrap();
        if let Some(l) = sums.get_mut(level) {
            *l += node.borrow().val;
        } else {
            sums.push(node.borrow().val);
        }

        Solution::dfs(sums, node.borrow().left.clone(), level + 1);
        Solution::dfs(sums, node.borrow().right.clone(), level + 1);
    }

    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sums = vec![];
        Solution::dfs(&mut sums, root, 0);
        sums.into_iter()
            .enumerate()
            .rev()
            .max_by_key(|x| x.1)
            .unwrap()
            .0 as i32
            + 1
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
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(-8))));
        let t3 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let t4 = TreeNode::new_with(7, t1, t2);
        let root = TreeNode::new_with(1, t4, t3);
        assert_eq!(Solution::max_level_sum(root), 2);
    }

    #[test]
    fn test2() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(98693))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode::new(-32127))));
        let t3 = TreeNode::new_with(-89388, None, t2);
        let t4 = TreeNode::new_with(10250, t1, t3);
        let root = TreeNode::new_with(989, None, t4);
        assert_eq!(Solution::max_level_sum(root), 2);
    }
}
