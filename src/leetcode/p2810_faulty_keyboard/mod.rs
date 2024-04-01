pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn final_string(s: String) -> String {
        let mut queue = VecDeque::with_capacity(s.len());
        let mut order = true;
        for c in s.chars() {
            if c == 'i' {
                order = !order;
                continue;
            }
            if order {
                queue.push_back(c);
            } else {
                queue.push_front(c);
            }
        }
        if order {
            queue.iter().collect()
        } else {
            queue.iter().rev().collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::final_string("string".to_string()), "rtsng");
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::final_string("poiinter".to_string()), "ponter");
    }
}
