pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; nums.len()];
        let mut pre_i = 1;
        let mut pre_j = 1;
        for i in 0..nums.len() {
            let j = nums.len() - 1 - i;
            result[i] *= pre_i;
            result[j] *= pre_j;
            pre_i *= nums[i];
            pre_j *= nums[j];
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
            Solution::product_except_self([1, 2, 3, 4].to_vec()),
            [24, 12, 8, 6]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::product_except_self([-1, 1, 0, -3, 3].to_vec()),
            [0, 0, 9, 0, 0]
        );
    }
}
