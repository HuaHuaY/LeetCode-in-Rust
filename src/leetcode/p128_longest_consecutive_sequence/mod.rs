pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let map = nums.into_iter().collect::<HashSet<_>>();

        let mut result = 0;
        for &num in &map {
            if map.contains(&(num - 1)) {
                continue;
            }

            let mut count = 1;
            while map.contains(&(num + count)) {
                count += 1;
            }
            result = result.max(count);
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
            Solution::longest_consecutive([100, 4, 200, 1, 3, 2].to_vec()),
            4
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::longest_consecutive([0, 3, 7, 2, 5, 8, 4, 6, 0, 1].to_vec()),
            9
        );
    }
}
