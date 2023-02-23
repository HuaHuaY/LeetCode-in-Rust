pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .filter(|e| *e != 0)
            .collect::<HashSet<i32>>()
            .len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_operations([1, 5, 0, 3, 5].to_vec()), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::minimum_operations([0].to_vec()), 0);
    }
}
