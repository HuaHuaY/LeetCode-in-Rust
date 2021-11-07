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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if let Some(_) = head.as_ref() {
            let mut head = head;
            let mut length = 1;
            let mut tail = head.as_mut();
            while let Some(_) = tail.as_ref().unwrap().next {
                tail = tail.unwrap().next.as_mut();
                length += 1;
            }

            let cut = (length - k % length) % length;
            if cut == 0 {
                head
            } else {
                let mut p = head.as_mut();
                for _ in 0..cut - 1 {
                    p = p.unwrap().next.as_mut();
                }
                let mut result = p.unwrap().next.take();
                p = result.as_mut();
                while let Some(_) = p.as_ref().unwrap().next {
                    p = p.unwrap().next.as_mut();
                    length += 1;
                }
                p.unwrap().next = head;
                result
            }
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
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode::new(5))),
                    })),
                })),
            })),
        }));
        let answer = Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode::new(3))),
                    })),
                })),
            })),
        }));
        assert_eq!(Solution::rotate_right(test, 2), answer);
    }

    #[test]
    fn test2() {
        let test = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode::new(2))),
            })),
        }));
        let answer = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode::new(1))),
            })),
        }));
        assert_eq!(Solution::rotate_right(test, 4), answer);
    }
}
