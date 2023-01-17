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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut tmp = Some(Box::new(ListNode::new(0)));
        tmp.as_mut().unwrap().next = head;
        let mut p = &mut tmp;
        while p.as_ref().unwrap().next.is_some()
            && p.as_ref().unwrap().next.as_ref().unwrap().next.is_some()
        {
            let mut a = p.as_mut().unwrap().next.take();
            let mut b = a.as_mut().unwrap().next.take();
            let c = b.as_mut().unwrap().next.take();
            a.as_mut().unwrap().next = c;
            p.as_mut().unwrap().next = b;
            p.as_mut().unwrap().next.as_mut().unwrap().next = a;
            p = &mut p.as_mut().unwrap().next.as_mut().unwrap().next;
        }
        tmp.unwrap().next.take()
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
        let mut l1 = Some(Box::new(ListNode::new(4)));
        l1 = ListNode::new_with(3, l1);
        l1 = ListNode::new_with(2, l1);
        l1 = ListNode::new_with(1, l1);

        let mut answer = Some(Box::new(ListNode::new(3)));
        answer = ListNode::new_with(4, answer);
        answer = ListNode::new_with(1, answer);
        answer = ListNode::new_with(2, answer);

        assert_eq!(Solution::swap_pairs(l1), answer);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::swap_pairs(None), None);
    }

    #[test]
    fn test3() {
        let l1 = Some(Box::new(ListNode::new(1)));

        let answer = Some(Box::new(ListNode::new(1)));

        assert_eq!(Solution::swap_pairs(l1), answer);
    }
}
