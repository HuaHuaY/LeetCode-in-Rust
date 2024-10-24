pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != 0 {
                nums.swap(i, j);
                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut v = [0, 1, 0, 3, 12].to_vec();
        Solution::move_zeroes(&mut v);
        assert_eq!(v, [1, 3, 12, 0, 0]);
    }

    #[test]
    fn test2() {
        let mut v = [0].to_vec();
        Solution::move_zeroes(&mut v);
        assert_eq!(v, [0]);
    }
}
