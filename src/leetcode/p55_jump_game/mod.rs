pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max = 0;
        for (i, num) in nums.into_iter().enumerate() {
            if i > max {
                return false;
            }
            max = max.max(i + num as usize);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::can_jump([2, 3, 1, 1, 4].to_vec()));
    }

    #[test]
    fn test2() {
        assert!(!Solution::can_jump([3, 2, 1, 0, 4].to_vec()));
    }
}
