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

impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut less = ListNode::new(0);
        let mut more_equal = ListNode::new(0);
        let mut p = &mut less;
        let mut q = &mut more_equal;
        while let Some(mut node) = head {
            head = node.next.take();
            if node.val < x {
                p.next = Some(node);
                p = p.next.as_mut().unwrap();
            } else {
                q.next = Some(node);
                q = q.next.as_mut().unwrap();
            }
        }
        p.next = more_equal.next.take();
        less.next.take()
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
        let mut l1 = Some(Box::new(ListNode::new(2)));
        l1 = ListNode::new_with(5, l1);
        l1 = ListNode::new_with(2, l1);
        l1 = ListNode::new_with(3, l1);
        l1 = ListNode::new_with(4, l1);
        l1 = ListNode::new_with(1, l1);
        let mut answer = Some(Box::new(ListNode::new(5)));
        answer = ListNode::new_with(3, answer);
        answer = ListNode::new_with(4, answer);
        answer = ListNode::new_with(2, answer);
        answer = ListNode::new_with(2, answer);
        answer = ListNode::new_with(1, answer);
        assert_eq!(Solution::partition(l1, 3), answer);
    }

    #[test]
    fn test2() {
        let mut l1 = Some(Box::new(ListNode::new(1)));
        l1 = ListNode::new_with(2, l1);
        let mut answer = Some(Box::new(ListNode::new(2)));
        answer = ListNode::new_with(1, answer);
        assert_eq!(Solution::partition(l1, 2), answer);
    }
}
