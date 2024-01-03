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
    pub fn remove_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut h: Option<Box<ListNode>> = None;
        while let Some(mut node) = head {
            head = node.next.take();
            while let Some(mut n) = h {
                if node.val > n.val {
                    h = n.next.take();
                } else {
                    h = Some(n);
                    break;
                }
            }
            node.next = h;
            h = Some(node);
        }
        while let Some(mut node) = h {
            h = node.next.take();
            node.next = head;
            head = Some(node);
        }
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
        let mut l1 = Some(Box::new(ListNode::new(8)));
        l1 = ListNode::new_with(3, l1);
        l1 = ListNode::new_with(13, l1);
        l1 = ListNode::new_with(2, l1);
        l1 = ListNode::new_with(5, l1);

        let mut answer = Some(Box::new(ListNode::new(8)));
        answer = ListNode::new_with(13, answer);

        assert_eq!(Solution::remove_nodes(l1), answer);
    }

    #[test]
    fn test2() {
        let mut l1 = Some(Box::new(ListNode::new(1)));
        l1 = ListNode::new_with(1, l1);
        l1 = ListNode::new_with(1, l1);
        l1 = ListNode::new_with(1, l1);

        let answer = l1.clone();

        assert_eq!(Solution::remove_nodes(l1), answer);
    }
}
