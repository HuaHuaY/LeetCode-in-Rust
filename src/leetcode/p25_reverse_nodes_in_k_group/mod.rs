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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut tmp = Some(Box::new(ListNode::new(0)));
        tmp.as_mut().unwrap().next = head;

        let mut count = 0;
        let mut p = &tmp;
        'label: loop {
            for _ in 0..k {
                if p.as_ref().unwrap().next.is_none() {
                    break 'label;
                }
                p = &p.as_ref().unwrap().next;
            }
            count += 1;
        }

        let mut p = &mut tmp;
        for _ in 0..count {
            let mut node = Vec::with_capacity(k as usize);
            let mut q = p.as_mut().unwrap().next.take();
            for _ in 0..k {
                node.push(q);
                q = node.last_mut().unwrap().as_mut().unwrap().next.take();
            }
            for _ in (0..k).rev() {
                p.as_mut().unwrap().next = node.pop().unwrap();
                p = &mut p.as_mut().unwrap().next;
            }
            p.as_mut().unwrap().next = q;
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
        let mut l1 = Some(Box::new(ListNode::new(5)));
        l1 = ListNode::new_with(4, l1);
        l1 = ListNode::new_with(3, l1);
        l1 = ListNode::new_with(2, l1);
        l1 = ListNode::new_with(1, l1);

        let mut answer = Some(Box::new(ListNode::new(5)));
        answer = ListNode::new_with(3, answer);
        answer = ListNode::new_with(4, answer);
        answer = ListNode::new_with(1, answer);
        answer = ListNode::new_with(2, answer);

        assert_eq!(Solution::reverse_k_group(l1, 2), answer);
    }

    #[test]
    fn test2() {
        let mut l1 = Some(Box::new(ListNode::new(5)));
        l1 = ListNode::new_with(4, l1);
        l1 = ListNode::new_with(3, l1);
        l1 = ListNode::new_with(2, l1);
        l1 = ListNode::new_with(1, l1);

        let mut answer = Some(Box::new(ListNode::new(5)));
        answer = ListNode::new_with(4, answer);
        answer = ListNode::new_with(1, answer);
        answer = ListNode::new_with(2, answer);
        answer = ListNode::new_with(3, answer);

        assert_eq!(Solution::reverse_k_group(l1, 3), answer);
    }
}
