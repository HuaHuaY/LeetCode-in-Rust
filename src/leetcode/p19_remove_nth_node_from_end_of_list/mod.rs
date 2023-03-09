pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut fast = &head;
        for _ in 0..n {
            fast = &fast.as_ref().unwrap().next;
        }
        let mut times = 0;
        while let Some(inner) = fast {
            fast = &inner.next;
            times += 1;
        }

        if times == 0 {
            return head.unwrap().next.take();
        }

        let mut slow = &mut head;
        for _ in 0..times - 1 {
            slow = &mut slow.as_mut().unwrap().next;
        }
        let mut tail = slow.as_mut().unwrap().next.take();
        slow.as_mut().unwrap().next = tail.as_mut().unwrap().next.take();
        head
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
        answer = ListNode::new_with(3, answer);
        answer = ListNode::new_with(2, answer);
        answer = ListNode::new_with(1, answer);

        assert_eq!(Solution::remove_nth_from_end(l1, 2), answer);
    }

    #[test]
    fn test2() {
        let l1 = Some(Box::new(ListNode::new(1)));

        assert_eq!(Solution::remove_nth_from_end(l1, 1), None);
    }

    #[test]
    fn test3() {
        let mut l1 = Some(Box::new(ListNode::new(2)));
        l1 = ListNode::new_with(1, l1);

        let answer = Some(Box::new(ListNode::new(1)));

        assert_eq!(Solution::remove_nth_from_end(l1, 1), answer);
    }
}
