pub struct Solution {}

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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut p1 = &l1;
        let mut p2 = &l2;
        let mut result = None;
        let mut tail = &mut result;

        let mut carry = 0;
        while let (Some(_), Some(_)) = (p1, p2) {
            let mut sum = p1.as_ref().unwrap().val + p2.as_ref().unwrap().val + carry;
            carry = sum / 10;
            sum %= 10;

            *tail = Some(Box::new(ListNode::new(sum)));
            tail = &mut tail.as_mut().unwrap().next;

            p1 = &p1.as_ref().unwrap().next;
            p2 = &p2.as_ref().unwrap().next;
        }

        if p2.is_some() {
            p1 = p2;
        }
        while p1.is_some() {
            let mut sum = p1.as_ref().unwrap().val + carry;
            carry = sum / 10;
            sum %= 10;

            *tail = Some(Box::new(ListNode::new(sum)));
            tail = &mut tail.as_mut().unwrap().next;

            p1 = &p1.as_ref().unwrap().next;
        }
        if carry == 1 {
            *tail = Some(Box::new(ListNode::new(1)));
        }
        result
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
        let mut l1 = Some(Box::new(ListNode::new(3)));
        l1 = ListNode::new_with(4, l1);
        l1 = ListNode::new_with(2, l1);

        let mut l2 = Some(Box::new(ListNode::new(4)));
        l2 = ListNode::new_with(6, l2);
        l2 = ListNode::new_with(5, l2);

        let mut answer = Some(Box::new(ListNode::new(8)));
        answer = ListNode::new_with(0, answer);
        answer = ListNode::new_with(7, answer);

        assert_eq!(Solution::add_two_numbers(l1, l2), answer);
    }

    #[test]
    fn test2() {
        let l1 = ListNode::new_with(0, None);
        let l2 = ListNode::new_with(0, None);
        let answer = ListNode::new_with(0, None);
        assert_eq!(Solution::add_two_numbers(l1, l2), answer);
    }

    #[test]
    fn test3() {
        let mut l1 = Some(Box::new(ListNode::new(9)));
        l1 = ListNode::new_with(9, l1);
        l1 = ListNode::new_with(9, l1);
        l1 = ListNode::new_with(9, l1);
        l1 = ListNode::new_with(9, l1);
        l1 = ListNode::new_with(9, l1);
        l1 = ListNode::new_with(9, l1);

        let mut l2 = Some(Box::new(ListNode::new(9)));
        l2 = ListNode::new_with(9, l2);
        l2 = ListNode::new_with(9, l2);
        l2 = ListNode::new_with(9, l2);

        let mut answer = Some(Box::new(ListNode::new(1)));
        answer = ListNode::new_with(0, answer);
        answer = ListNode::new_with(0, answer);
        answer = ListNode::new_with(0, answer);
        answer = ListNode::new_with(9, answer);
        answer = ListNode::new_with(9, answer);
        answer = ListNode::new_with(9, answer);
        answer = ListNode::new_with(8, answer);

        assert_eq!(Solution::add_two_numbers(l1, l2), answer);
    }
}
