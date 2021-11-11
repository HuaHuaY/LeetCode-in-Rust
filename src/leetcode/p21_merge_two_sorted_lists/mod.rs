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
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut p = &mut head;
        while l1 != None && l2 != None {
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

    #[test]
    fn test1() {
        let l1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 4, next: None })),
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(list, Solution::merge_two_lists(l1, l2));
    }

    #[test]
    fn test2() {
        let l1 = None;
        let l2 = None;
        let list = None;
        assert_eq!(list, Solution::merge_two_lists(l1, l2));
    }

    #[test]
    fn test3() {
        let l1 = None;
        let l2 = Some(Box::new(ListNode { val: 0, next: None }));
        let list = Some(Box::new(ListNode { val: 0, next: None }));
        assert_eq!(list, Solution::merge_two_lists(l1, l2));
    }
}
