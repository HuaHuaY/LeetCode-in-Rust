pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::with_capacity(nums.len());
        let mut result = 0;
        nums.iter().fold(0, |mut acc, x| {
            map.entry(acc).and_modify(|x| *x += 1).or_insert(1);
            acc += *x;
            if let Some(v) = map.get(&(acc - k)) {
                result += *v;
            }
            acc
        });
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::subarray_sum([1, 1, 1].to_vec(), 2), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::subarray_sum([1, 2, 3].to_vec(), 3), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::subarray_sum([1].to_vec(), 0), 0);
    }
}
