pub struct Solution;

impl Solution {
    fn merge(nums: &mut [i32], arr1: &[i32], arr2: &[i32]) {
        let mut i = 0;
        let mut j = 0;
        while i < arr1.len() && j < arr2.len() {
            if arr1[i] < arr2[j] {
                nums[i + j] = arr1[i];
                i += 1;
            } else {
                nums[i + j] = arr2[j];
                j += 1;
            }
        }
        while i < arr1.len() {
            nums[i + j] = arr1[i];
            i += 1;
        }
        while j < arr2.len() {
            nums[i + j] = arr2[j];
            j += 1;
        }
    }

    pub fn merge_sort(nums: &mut [i32]) {
        if nums.len() <= 1 {
            return;
        }
        let mut arr1 = nums[0..nums.len() / 2].to_vec();
        let mut arr2 = nums[nums.len() / 2..nums.len()].to_vec();
        Solution::merge_sort(&mut arr1);
        Solution::merge_sort(&mut arr2);
        Solution::merge(nums, &arr1, &arr2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = [2, 4, 1, 3, 5];
        Solution::merge_sort(&mut nums);
        assert_eq!(nums, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test2() {
        let mut nums = [1, 2, 0, 0, 2];
        Solution::merge_sort(&mut nums);
        assert_eq!(nums, [0, 0, 1, 2, 2]);
    }

    #[test]
    fn test3() {
        let mut nums = [];
        Solution::merge_sort(&mut nums);
        assert_eq!(nums, []);
    }

    #[test]
    fn test4() {
        let mut nums = [0];
        Solution::merge_sort(&mut nums);
        assert_eq!(nums, [0]);
    }
}
