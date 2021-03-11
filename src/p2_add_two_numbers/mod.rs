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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut p1 = &l1;
        let mut p2 = &l2;
        let mut result = None;
        let mut tail = &mut result;

        let mut carry = 0;
        while let (Some(_), Some(_)) = (p1, p2) {
            let mut sum = p1.as_ref().unwrap().val + p2.as_ref().unwrap().val + carry;
            carry = sum / 10;
            sum %= 10;

            *tail = Some(Box::new(ListNode::new(sum)));
            tail = &mut tail.as_mut().unwrap().next;

            p1 = &p1.as_ref().unwrap().next;
            p2 = &p2.as_ref().unwrap().next;
        }

        if let Some(_) = p2 {
            p1 = p2;
        }
        while let Some(_) = p1 {
            let mut sum = p1.as_ref().unwrap().val + carry;
            carry = sum / 10;
            sum %= 10;

            *tail = Some(Box::new(ListNode::new(sum)));
            tail = &mut tail.as_mut().unwrap().next;

            p1 = &p1.as_ref().unwrap().next;
        }
        if carry == 1 {
            *tail = Some(Box::new(ListNode::new(1)));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));

        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));

        let answer = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None })),
            })),
        }));
        assert_eq!(Solution::add_two_numbers(l1, l2), answer);
    }

    #[test]
    fn test2() {
        let l1 = Some(Box::new(ListNode { val: 0, next: None }));

        let l2 = Some(Box::new(ListNode { val: 0, next: None }));

        let answer = Some(Box::new(ListNode { val: 0, next: None }));

        assert_eq!(Solution::add_two_numbers(l1, l2), answer);
    }

    #[test]
    fn test3() {
        let l1 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode { val: 9, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        }));

        let l2 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode { val: 9, next: None })),
                })),
            })),
        }));

        let answer = Some(Box::new(ListNode {
            val: 8,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 0,
                            next: Some(Box::new(ListNode {
                                val: 0,
                                next: Some(Box::new(ListNode {
                                    val: 0,
                                    next: Some(Box::new(ListNode { val: 1, next: None })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(Solution::add_two_numbers(l1, l2), answer);
    }
}
