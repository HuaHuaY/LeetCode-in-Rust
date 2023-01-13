pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

use std::collections::BinaryHeap;

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::from(lists);
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut p = &mut head;
        while !heap.is_empty() {
            if let smallest @ Some(_) = heap.pop().unwrap() {
                p.as_mut().unwrap().next = smallest;
                p = &mut p.as_mut().unwrap().next;
                if p.as_ref().unwrap().next.is_some() {
                    heap.push(p.as_mut().unwrap().next.take())
                }
            }
        }
        head.unwrap().next.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl ListNode {
        #[inline]
        fn new_with(val: i32, next: Option<Box<ListNode>>) -> Option<Box<Self>> {
            Some(Box::new(ListNode { next, val }))
        }
    }

    #[test]
    fn test1() {
        let mut l1 = Some(Box::new(ListNode::new(5)));
        l1 = ListNode::new_with(4, l1);
        l1 = ListNode::new_with(1, l1);

        let mut l2 = Some(Box::new(ListNode::new(4)));
        l2 = ListNode::new_with(3, l2);
        l2 = ListNode::new_with(1, l2);

        let mut l3 = Some(Box::new(ListNode::new(6)));
        l3 = ListNode::new_with(2, l3);

        let mut answer = Some(Box::new(ListNode::new(6)));
        answer = ListNode::new_with(5, answer);
        answer = ListNode::new_with(4, answer);
        answer = ListNode::new_with(4, answer);
        answer = ListNode::new_with(3, answer);
        answer = ListNode::new_with(2, answer);
        answer = ListNode::new_with(1, answer);
        answer = ListNode::new_with(1, answer);

        assert_eq!(Solution::merge_k_lists(vec![l1, l2, l3]), answer);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::merge_k_lists(vec![]), None);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::merge_k_lists(vec![None]), None);
    }
}
