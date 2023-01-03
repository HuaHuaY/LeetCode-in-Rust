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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        if let Some(mut p) = head.as_mut() {
            while p.next.as_ref().is_some() {
                if p.next.as_ref().unwrap().val == p.val {
                    p.next = p.next.as_mut().unwrap().next.take();
                } else {
                    p = p.next.as_mut().unwrap();
                }
            }
            head
        } else {
            None
        }
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
        let mut test = Some(Box::new(ListNode::new(2)));
        test = ListNode::new_with(1, test);
        test = ListNode::new_with(1, test);

        let mut answer = Some(Box::new(ListNode::new(2)));
        answer = ListNode::new_with(1, answer);

        assert_eq!(Solution::delete_duplicates(test), answer);
    }

    #[test]
    fn test2() {
        let mut test = Some(Box::new(ListNode::new(3)));
        test = ListNode::new_with(3, test);
        test = ListNode::new_with(2, test);
        test = ListNode::new_with(1, test);
        test = ListNode::new_with(1, test);

        let mut answer = Some(Box::new(ListNode::new(3)));
        answer = ListNode::new_with(2, answer);
        answer = ListNode::new_with(1, answer);

        assert_eq!(Solution::delete_duplicates(test), answer);
    }
}
