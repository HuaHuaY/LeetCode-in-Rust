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
    pub fn next_larger_nodes(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut i = 0;
        let mut result = vec![];
        let mut stack: Vec<(usize, i32)> = vec![];
        while let Some(mut node) = head {
            head = node.next.take();
            result.push(0);
            while let Some(last) = stack.last() {
                if (*last).1 < node.val {
                    result[last.0] = node.val;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push((i, node.val));
            i += 1;
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
        let mut l1 = Some(Box::new(ListNode::new(5)));
        l1 = ListNode::new_with(1, l1);
        l1 = ListNode::new_with(2, l1);

        assert_eq!(Solution::next_larger_nodes(l1), [5, 5, 0]);
    }

    #[test]
    fn test2() {
        let mut l1 = Some(Box::new(ListNode::new(5)));
        l1 = ListNode::new_with(3, l1);
        l1 = ListNode::new_with(4, l1);
        l1 = ListNode::new_with(7, l1);
        l1 = ListNode::new_with(2, l1);

        assert_eq!(Solution::next_larger_nodes(l1), [7, 0, 5, 5, 0]);
    }
}
