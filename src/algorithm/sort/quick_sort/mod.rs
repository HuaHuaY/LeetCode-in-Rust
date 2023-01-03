pub struct Solution;

impl Solution {
    fn sort(nums: &mut [i32], left: isize, right: isize) {
        if left >= right {
            return;
        }

        let mid = (right - left) / 2 + left;
        nums.swap(left as usize, mid as usize);

        let mut i = left as usize;
        let mut j = right as usize;
        while i < j {
            while i < j && nums[j] > nums[i] {
                j -= 1;
            }
            nums.swap(i, j);
            while i < j && nums[j] >= nums[i] {
                i += 1;
            }
            nums.swap(i, j);
        }
        Solution::sort(nums, left, i as isize - 1);
        Solution::sort(nums, i as isize + 1, right);
    }

    pub fn quick_sort(nums: &mut [i32]) {
        Solution::sort(nums, 0, nums.len() as isize - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = [2, 4, 1, 3, 5];
        Solution::quick_sort(&mut nums);
        assert_eq!(nums, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test2() {
        let mut nums = [1, 2, 0, 0, 2];
        Solution::quick_sort(&mut nums);
        assert_eq!(nums, [0, 0, 1, 2, 2]);
    }

    #[test]
    fn test3() {
        let mut nums: [i32; 0] = [];
        Solution::quick_sort(&mut nums);
        assert_eq!(nums, []);
    }

    #[test]
    fn test4() {
        let mut nums = [0];
        Solution::quick_sort(&mut nums);
        assert_eq!(nums, [0]);
    }
}
