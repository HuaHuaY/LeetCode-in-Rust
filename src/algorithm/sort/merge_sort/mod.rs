pub struct Solution {}

impl Solution {
    fn merge(nums: &mut Vec<i32>, vec1: Vec<i32>, vec2: Vec<i32>) {
        let mut i = 0;
        let mut j = 0;
        while i < vec1.len() && j < vec2.len() {
            if vec1[i] < vec2[j] {
                nums[i + j] = vec1[i];
                i += 1;
            } else {
                nums[i + j] = vec2[j];
                j += 1;
            }
        }
        while i < vec1.len() {
            nums[i + j] = vec1[i];
            i += 1;
        }
        while j < vec2.len() {
            nums[i + j] = vec2[j];
            j += 1;
        }
    }

    pub fn merge_sort(nums: &mut Vec<i32>) {
        if nums.len() <= 1 {
            return;
        }
        let mut vec1 = nums[0..nums.len() / 2].to_vec();
        let mut vec2 = nums[nums.len() / 2..nums.len()].to_vec();
        Solution::merge_sort(&mut vec1);
        Solution::merge_sort(&mut vec2);
        Solution::merge(nums, vec1, vec2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = vec![2, 4, 1, 3, 5];
        Solution::merge_sort(&mut nums);
        assert_eq!(vec![1, 2, 3, 4, 5], nums);
    }

    #[test]
    fn test2() {
        let mut nums = vec![1, 2, 0, 0, 2];
        Solution::merge_sort(&mut nums);
        assert_eq!(vec![0, 0, 1, 2, 2], nums);
    }

    #[test]
    fn test3() {
        let mut nums: Vec<i32> = vec![];
        Solution::merge_sort(&mut nums);
        assert_eq!(vec![] as Vec<i32>, nums);
    }

    #[test]
    fn test4() {
        let mut nums = vec![0];
        Solution::merge_sort(&mut nums);
        assert_eq!(vec![0], nums);
    }
}
