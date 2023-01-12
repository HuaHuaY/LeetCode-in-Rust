pub struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let mut result = i32::MAX;
        nums.sort_unstable();
        for i in 0..nums.len() {
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if (sum - target).abs() < (result - target).abs() {
                    result = sum;
                }

                if sum < target {
                    j += 1;
                } else {
                    k -= 1;
                }
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
        assert_eq!(Solution::three_sum_closest([-1, 2, 1, -4].to_vec(), 1), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::three_sum_closest([0, 0, 0].to_vec(), 1), 0);
    }
}
