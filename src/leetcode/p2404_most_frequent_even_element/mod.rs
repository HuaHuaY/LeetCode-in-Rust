pub struct Solution;

use std::cmp::Ordering;
use std::collections::HashMap;

impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut min = -1;
        let mut count = 0;
        for num in nums {
            if num & 1 == 0 {
                let v = map.entry(num).or_insert(0);
                *v += 1;
                match count.cmp(v) {
                    Ordering::Less => {
                        count = *v;
                        min = num;
                    }
                    Ordering::Equal if num < min => {
                        count = *v;
                        min = num;
                    }
                    _ => (),
                }
            }
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::most_frequent_even([0, 1, 2, 2, 4, 4, 1].to_vec()),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::most_frequent_even([4, 4, 4, 9, 2, 4].to_vec()), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::most_frequent_even([29, 47, 21, 41, 13, 37, 25, 7].to_vec()),
            -1
        );
    }
}
