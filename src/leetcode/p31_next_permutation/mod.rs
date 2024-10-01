pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut i = nums.len() - 1;
        while i > 0 && nums[i - 1] >= nums[i] {
            i -= 1;
        }
        if i == 0 {
            nums.reverse();
            return;
        }
        let mut j = nums.len() - 1;
        while j > i - 1 && nums[j] <= nums[i - 1] {
            j -= 1;
        }
        nums.swap(i - 1, j);
        nums[i..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, [1, 3, 2]);
    }

    #[test]
    fn test2() {
        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, [1, 2, 3]);
    }

    #[test]
    fn test3() {
        let mut nums = vec![1, 1, 5];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, [1, 5, 1]);
    }
}
