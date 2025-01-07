pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut max = 0;
        num.bytes()
            .fold(VecDeque::with_capacity(3), |mut queue, i| {
                if let Some(&j) = queue.front() {
                    if i != j {
                        queue.clear();
                    }
                }
                queue.push_back(i);
                if queue.len() == 3 {
                    let j = queue.pop_front().unwrap();
                    max = max.max(j);
                }
                queue
            });
        if max == 0 {
            "".to_string()
        } else {
            String::from_utf8(vec![max, max, max]).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::largest_good_integer("6777133339".to_string()),
            "777"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::largest_good_integer("2300019".to_string()), "000");
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::largest_good_integer("42352338".to_string()), "");
    }
}
