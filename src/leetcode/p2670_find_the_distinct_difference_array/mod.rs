pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
        let mut set = HashSet::new();
        let mut result = vec![0; nums.len()];
        for (i, &num) in nums.iter().enumerate().rev() {
            result[i] = set.len() as i32;
            set.insert(num);
        }
        set.clear();
        for (num, res) in nums.into_iter().zip(result.iter_mut()) {
            set.insert(num);
            *res = set.len() as i32 - *res;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::distinct_difference_array([1, 2, 3, 4, 5].to_vec()),
            [-3, -1, 1, 3, 5]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::distinct_difference_array([3, 2, 3, 4, 2].to_vec()),
            [-2, -1, 0, 2, 3]
        );
    }
}
