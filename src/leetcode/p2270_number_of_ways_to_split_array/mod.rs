pub struct Solution;

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().copied().map(i64::from).sum::<i64>();
        let mut result = 0;
        let mut left = 0;
        let mut right = sum;
        for &num in &nums[..nums.len() - 1] {
            left += num as i64;
            right -= num as i64;
            if left >= right {
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::ways_to_split_array([10, 4, -8, 7].to_vec()), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::ways_to_split_array([2, 3, 1, 0].to_vec()), 2);
    }
}
