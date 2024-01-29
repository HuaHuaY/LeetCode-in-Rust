pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn minimum_seconds(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut map = HashMap::new();
        for (i, num) in nums.into_iter().enumerate() {
            map.entry(num)
                .and_modify(|v: &mut Vec<usize>| v.push(i))
                .or_insert(vec![i]);
        }
        let mut result = i32::MAX;
        for v in map.into_values() {
            let mut max = v.first().unwrap() + len - v.last().unwrap();
            for t in v.windows(2) {
                max = max.max(t[1] - t[0]);
            }
            result = result.min(max as i32);
        }
        result / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_seconds([1, 2, 1, 2].to_vec()), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::minimum_seconds([2, 1, 3, 3, 2].to_vec()), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::minimum_seconds([5, 5, 5, 5].to_vec()), 0);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::minimum_seconds([8, 8, 9, 10, 9].to_vec()), 1);
    }
}
