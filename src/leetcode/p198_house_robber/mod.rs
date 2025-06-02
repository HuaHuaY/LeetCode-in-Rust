pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut first = nums[0];
        let mut second = nums[1].max(nums[0]);
        for num in nums.into_iter().skip(2) {
            let current = num + first;
            first = second;
            second = second.max(current);
        }
        second
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::rob([1, 2, 3, 1].to_vec()), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::rob([2, 7, 9, 3, 1].to_vec()), 12);
    }
}
