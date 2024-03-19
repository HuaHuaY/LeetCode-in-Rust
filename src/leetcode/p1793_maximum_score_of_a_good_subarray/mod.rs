pub struct Solution;

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        let mut left = k - 1;
        let mut right = k + 1;
        let mut result = nums[k as usize];
        let mut min = nums[k as usize];
        while left >= 0 || right < len as i32 {
            while left >= 0 && nums[left as usize] >= min {
                left -= 1;
            }
            while right < len as i32 && nums[right as usize] >= min {
                right += 1;
            }
            result = result.max(min * (right - left - 1));
            min = 0;
            if left >= 0 {
                min = min.max(nums[left as usize]);
            }
            if right < len as i32 {
                min = min.max(nums[right as usize]);
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
        assert_eq!(Solution::maximum_score([1, 4, 3, 7, 4, 5].to_vec(), 3), 15);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::maximum_score([5, 5, 4, 5, 4, 1, 1, 1].to_vec(), 0),
            20
        );
    }
}
