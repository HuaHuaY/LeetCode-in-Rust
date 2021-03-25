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
        let mut head = ListNode { val: 0, next: head };
        let mut p = &mut head;
        while let Some(mut p_next) = p.next.as_mut() {
            if let Some(_) = p_next.next.as_mut() {
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
    #[test]
    fn test1() {
        let test = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode::new(5))),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        let answer = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(5))),
            })),
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
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode::new(3))),
                    })),
                })),
            })),
        }));
        let answer = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        }));
        assert_eq!(Solution::delete_duplicates(test), answer);
    }
}
