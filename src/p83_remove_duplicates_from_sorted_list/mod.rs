pub struct Solution {}

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
            while let Some(_) = p.next.as_ref() {
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
    #[test]
    fn test1() {
        let test = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode::new(2))),
            })),
        }));
        let answer = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode::new(2))),
        }));
        assert_eq!(Solution::delete_duplicates(test), answer);
    }

    #[test]
    fn test2() {
        let test = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode::new(3))),
                    })),
                })),
            })),
        }));
        let answer = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(3))),
            })),
        }));
        assert_eq!(Solution::delete_duplicates(test), answer);
    }
}
