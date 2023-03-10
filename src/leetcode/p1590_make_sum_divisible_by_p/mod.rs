pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let mut x = 0;
        for num in &nums {
            x = (x + *num) % p;
        }
        if x == 0 {
            return 0;
        }

        let mut min = i32::MAX;
        let mut y = 0;
        let len = nums.len() as i32;
        let mut map = HashMap::new();
        map.insert(0, -1);
        for (i, num) in nums.into_iter().enumerate() {
            y = (y + num) % p;
            if let Some(idx) = map.get(&((y - x + p) % p)) {
                min = min.min(i as i32 - idx);
            }
            map.insert(y, i as i32);
        }
        if min < len {
            min
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_subarray([3, 1, 4, 2].to_vec(), 6), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_subarray([6, 3, 5, 2].to_vec(), 9), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::min_subarray([1, 2, 3].to_vec(), 3), 0);
    }
}
