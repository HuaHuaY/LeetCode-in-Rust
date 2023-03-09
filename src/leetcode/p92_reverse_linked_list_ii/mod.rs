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
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut i = 0;
        let mut h = ListNode::new(0);
        h.next = head;
        let mut p = &mut h;
        while i < left - 1 {
            p = p.next.as_mut().unwrap();
            i += 1;
        }
        let mut q = p.next.take();
        while i < right {
            let t = q.as_mut().unwrap().next.take();
            q.as_mut().unwrap().next = p.next.take();
            p.next = q;
            q = t;
            i += 1;
        }
        while p.next.is_some() {
            p = p.next.as_mut().unwrap();
        }
        p.next = q;
        h.next.take()
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
        l1 = ListNode::new_with(3, l1);
        l1 = ListNode::new_with(2, l1);
        l1 = ListNode::new_with(1, l1);

        let mut answer = Some(Box::new(ListNode::new(5)));
        answer = ListNode::new_with(2, answer);
        answer = ListNode::new_with(3, answer);
        answer = ListNode::new_with(4, answer);
        answer = ListNode::new_with(1, answer);
        assert_eq!(Solution::reverse_between(l1, 2, 4), answer);
    }

    #[test]
    fn test2() {
        let l1 = Some(Box::new(ListNode::new(5)));
        let answer = Some(Box::new(ListNode::new(5)));
        assert_eq!(Solution::reverse_between(l1, 1, 1), answer);
    }
}
