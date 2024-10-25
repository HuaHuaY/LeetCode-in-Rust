pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        nums.rotate_right(k as usize % len);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = [1, 2, 3, 4, 5, 6, 7].to_vec();
        Solution::rotate(&mut nums, 3);
        assert_eq!(nums, [5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn test2() {
        let mut nums = [-1, -100, 3, 99].to_vec();
        Solution::rotate(&mut nums, 2);
        assert_eq!(nums, [3, 99, -1, -100]);
    }
}
