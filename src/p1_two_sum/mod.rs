pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for i in 0..nums.len() {
            match map.get(&(target - nums[i])) {
                Some(&y) => return [y as i32, i as i32].to_vec(),
                None => {
                    map.insert(nums[i], i);
                }
            }
        }

        [].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), [0, 1]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), [1, 2]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), [0, 1]);
    }
}
