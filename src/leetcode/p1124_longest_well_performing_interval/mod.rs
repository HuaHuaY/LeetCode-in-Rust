pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut pre_sum = 0;
        let mut max_len = 0;
        for (idx, hour) in hours.into_iter().enumerate() {
            pre_sum += if hour > 8 { 1 } else { -1 };
            if pre_sum > 0 {
                max_len = max_len.max(idx as i32 + 1);
            } else if let Some(&pre_idx) = map.get(&(pre_sum - 1)) {
                max_len = max_len.max((idx - pre_idx) as i32);
            }
            map.entry(pre_sum).or_insert(idx);
        }
        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_wpi([9, 9, 6, 0, 6, 6, 9].to_vec()), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::longest_wpi([6, 6, 6].to_vec()), 0);
    }
}
