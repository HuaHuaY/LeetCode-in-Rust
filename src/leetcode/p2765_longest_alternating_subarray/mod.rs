pub struct Solution;

impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let mut result = -1;
        let mut count = 0;
        let mut op = 1;
        let mut diff: i32;
        for (num1, num2) in nums.iter().zip(&nums[1..]) {
            diff = *num2 - *num1;
            if diff != op {
                count = 0;
                op = 1;
            }
            if diff == op {
                count += 1;
                result = result.max(count + 1);
                op = -op;
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
        assert_eq!(Solution::alternating_subarray([2, 3, 4, 3, 4].to_vec()), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::alternating_subarray([4, 5, 6].to_vec()), 2);
    }
}
