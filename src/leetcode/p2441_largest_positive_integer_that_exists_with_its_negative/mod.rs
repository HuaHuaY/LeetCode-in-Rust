pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut max = -1;
        let mut set = HashSet::new();
        for i in nums {
            set.insert(i);
            if set.contains(&(-i)) {
                max = max.max(i.abs());
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_max_k([-1, 2, -3, 3].to_vec()), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_max_k([-1, 10, 6, 7, -7, 1].to_vec()), 7);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::find_max_k([-10, 8, 6, 7, -2, -3].to_vec()), -1);
    }
}
