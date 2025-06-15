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
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        let mut count = 0;
        let mut slow = &head;
        while let Some(ref node) = slow {
            count += 1;
            slow = &node.next;
        }
        let mut fast = &mut head;
        for _ in 0..(count + 1) / 2 {
            fast = &mut fast.as_mut().unwrap().next;
        }
        let mut tail1 = fast.take();
        let mut head2 = None;
        while let Some(mut node) = tail1 {
            tail1 = node.next.take();
            node.next = head2;
            head2 = Some(node);
        }
        while let Some(mut node) = head2 {
            if let Some(ref mut head_node) = head {
                if node.val != head_node.val {
                    return false;
                }
                head = head_node.next.take();
            } else {
                return false;
            }
            head2 = node.next.take();
        }
        true
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
        let mut l1 = Some(Box::new(ListNode::new(1)));
        l1 = ListNode::new_with(2, l1);
        l1 = ListNode::new_with(2, l1);
        l1 = ListNode::new_with(1, l1);

        assert!(Solution::is_palindrome(l1));
    }

    #[test]
    fn test2() {
        let mut l1 = Some(Box::new(ListNode::new(2)));
        l1 = ListNode::new_with(1, l1);

        assert!(!Solution::is_palindrome(l1));
    }
}
