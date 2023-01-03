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
        let mut head = ListNode { val: 0, next: head };
        let mut p = &mut head;
        while let Some(mut p_next) = p.next.as_mut() {
            if p_next.next.as_mut().is_some() {
                if p_next.val == p_next.next.as_ref().unwrap().val {
                    while let Some(p_next_next) = p_next.next.as_mut() {
                        if p_next.val == p_next_next.val {
                            p_next.next = p_next_next.next.take();
                        } else {
                            break;
                        }
                    }
                    p.next = p_next.next.take();
                } else {
                    p = p.next.as_mut().unwrap();
                }
            } else {
                break;
            }
        }
        head.next
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
        let mut test = Some(Box::new(ListNode::new(5)));
        test = ListNode::new_with(4, test);
        test = ListNode::new_with(4, test);
        test = ListNode::new_with(3, test);
        test = ListNode::new_with(3, test);
        test = ListNode::new_with(2, test);
        test = ListNode::new_with(1, test);

        let mut answer = Some(Box::new(ListNode::new(5)));
        answer = ListNode::new_with(2, answer);
        answer = ListNode::new_with(1, answer);

        assert_eq!(Solution::delete_duplicates(test), answer);
    }

    #[test]
    fn test2() {
        let mut test = Some(Box::new(ListNode::new(3)));
        test = ListNode::new_with(2, test);
        test = ListNode::new_with(1, test);
        test = ListNode::new_with(1, test);
        test = ListNode::new_with(1, test);

        let mut answer = Some(Box::new(ListNode::new(3)));
        answer = ListNode::new_with(2, answer);

        assert_eq!(Solution::delete_duplicates(test), answer);
    }
}
