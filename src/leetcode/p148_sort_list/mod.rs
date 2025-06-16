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
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let merge = |mut l1: Option<Box<ListNode>>,
                     mut l2: Option<Box<ListNode>>|
         -> Option<Box<ListNode>> {
            let mut result = None;
            let mut tail = &mut result;
            loop {
                match (l1, l2) {
                    (None, None) => return result,
                    (Some(node), None) => {
                        let _ = tail.insert(node);
                        return result;
                    }
                    (None, Some(node)) => {
                        let _ = tail.insert(node);
                        return result;
                    }
                    (Some(mut node1), Some(mut node2)) => {
                        if node1.val < node2.val {
                            l1 = node1.next.take();
                            l2 = Some(node2);
                            tail = &mut tail.insert(node1).next;
                        } else {
                            l1 = Some(node1);
                            l2 = node2.next.take();
                            tail = &mut tail.insert(node2).next;
                        }
                    }
                }
            }
        };
        let take = |mut head: Option<Box<ListNode>>,
                    length: usize|
         -> (Option<Box<ListNode>>, Option<Box<ListNode>>, bool) {
            let mut result = None;
            let mut tail = &mut result;
            for _ in 0..length {
                if let Some(mut node) = head {
                    head = node.next.take();
                    tail = &mut tail.insert(node).next;
                } else {
                    return (result, head, true);
                }
            }
            (result, head, false)
        };

        let mut length = 1;
        loop {
            let tail = head.take();
            let mut head_tail = &mut head;

            let (l1, tail, end) = take(tail, length);
            if end {
                return l1;
            }
            let (l2, mut tail, _) = take(tail, length);
            *head_tail = merge(l1, l2);
            while head_tail.is_some() {
                head_tail = &mut head_tail.as_mut().unwrap().next;
            }

            while tail.is_some() {
                let (l1, t, _) = take(tail, length);
                let (l2, t, _) = take(t, length);
                tail = t;
                *head_tail = merge(l1, l2);
                while head_tail.is_some() {
                    head_tail = &mut head_tail.as_mut().unwrap().next;
                }
            }
            length *= 2;
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
        let mut l1 = Some(Box::new(ListNode::new(3)));
        l1 = ListNode::new_with(1, l1);
        l1 = ListNode::new_with(2, l1);
        l1 = ListNode::new_with(4, l1);

        let mut answer = Some(Box::new(ListNode::new(4)));
        answer = ListNode::new_with(3, answer);
        answer = ListNode::new_with(2, answer);
        answer = ListNode::new_with(1, answer);

        assert_eq!(Solution::sort_list(l1), answer);
    }

    #[test]
    fn test2() {
        let mut l1 = Some(Box::new(ListNode::new(0)));
        l1 = ListNode::new_with(4, l1);
        l1 = ListNode::new_with(3, l1);
        l1 = ListNode::new_with(5, l1);
        l1 = ListNode::new_with(-1, l1);

        let mut answer = Some(Box::new(ListNode::new(5)));
        answer = ListNode::new_with(4, answer);
        answer = ListNode::new_with(3, answer);
        answer = ListNode::new_with(0, answer);
        answer = ListNode::new_with(-1, answer);

        assert_eq!(Solution::sort_list(l1), answer);
    }
}
