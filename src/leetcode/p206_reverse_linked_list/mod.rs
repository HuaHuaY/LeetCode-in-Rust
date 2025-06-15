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
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut answer = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = answer;
            answer = Some(node);
        }
        answer
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

        let mut answer = Some(Box::new(ListNode::new(1)));
        answer = ListNode::new_with(2, answer);
        answer = ListNode::new_with(3, answer);
        answer = ListNode::new_with(4, answer);
        answer = ListNode::new_with(5, answer);

        assert_eq!(Solution::reverse_list(l1), answer);
    }

    #[test]
    fn test2() {
        let mut l1 = Some(Box::new(ListNode::new(2)));
        l1 = ListNode::new_with(1, l1);

        let mut answer = Some(Box::new(ListNode::new(1)));
        answer = ListNode::new_with(2, answer);

        assert_eq!(Solution::reverse_list(l1), answer);
    }
}
