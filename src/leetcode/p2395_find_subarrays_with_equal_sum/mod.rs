pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for i in 0..nums.len() - 1 {
            if set.contains(&(nums[i] + nums[i + 1])) {
                return true;
            } else {
                set.insert(nums[i] + nums[i + 1]);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::find_subarrays([4, 2, 4].to_vec()));
    }

    #[test]
    fn test2() {
        assert!(!Solution::find_subarrays([1, 2, 3, 4, 5].to_vec()));
    }

    #[test]
    fn test3() {
        assert!(Solution::find_subarrays([0, 0, 0].to_vec()));
    }
}
