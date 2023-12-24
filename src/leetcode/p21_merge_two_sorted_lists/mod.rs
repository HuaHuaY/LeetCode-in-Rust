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
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut p = &mut head;
        while l1.is_some() && l2.is_some() {
            if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
                let tmp = l1.as_mut().unwrap().next.take();
                p.as_mut().unwrap().next = l1;
                p = &mut p.as_mut().unwrap().next;
                l1 = tmp;
            } else {
                let tmp = l2.as_mut().unwrap().next.take();
                p.as_mut().unwrap().next = l2;
                p = &mut p.as_mut().unwrap().next;
                l2 = tmp;
            }
        }
        p.as_mut().unwrap().next = if l1.is_some() { l1 } else { l2 };
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
        let mut l1 = Some(Box::new(ListNode::new(4)));
        l1 = ListNode::new_with(2, l1);
        l1 = ListNode::new_with(1, l1);

        let mut l2 = Some(Box::new(ListNode::new(4)));
        l2 = ListNode::new_with(3, l2);
        l2 = ListNode::new_with(1, l2);

        let mut answer = Some(Box::new(ListNode::new(4)));
        answer = ListNode::new_with(4, answer);
        answer = ListNode::new_with(3, answer);
        answer = ListNode::new_with(2, answer);
        answer = ListNode::new_with(1, answer);
        answer = ListNode::new_with(1, answer);

        assert_eq!(Solution::merge_two_lists(l1, l2), answer);
    }

    #[test]
    fn test2() {
        let l1 = None;
        let l2 = None;
        let answer = None;
        assert_eq!(Solution::merge_two_lists(l1, l2), answer);
    }

    #[test]
    fn test3() {
        let l1 = None;
        let l2 = Some(Box::new(ListNode::new(0)));
        let answer = Some(Box::new(ListNode::new(0)));
        assert_eq!(Solution::merge_two_lists(l1, l2), answer);
    }
}
