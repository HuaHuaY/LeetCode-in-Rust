pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut max_idx = 0;
        let mut end = 0;
        let mut result = 0;
        for (i, num) in nums.iter().enumerate().take(nums.len() - 1) {
            max_idx = max_idx.max(i + *num as usize);
            if i == end {
                end = max_idx;
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
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }
}
