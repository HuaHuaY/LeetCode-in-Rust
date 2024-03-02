pub struct Solution;

impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true;
        dp[2] = nums[0] == nums[1];
        for i in 3..=n {
            dp[i] = dp[i - 2] && nums[i - 2] == nums[i - 1];
            if !dp[i] {
                dp[i] = dp[i - 3]
                    && ((nums[i - 3] == nums[i - 2] && nums[i - 2] == nums[i - 1])
                        || (nums[i - 3] + 1 == nums[i - 2] && nums[i - 2] + 1 == nums[i - 1]));
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::valid_partition([4, 4, 4, 5, 6].to_vec()));
    }

    #[test]
    fn test2() {
        assert!(!Solution::valid_partition([1, 1, 1, 2].to_vec()));
    }
}
