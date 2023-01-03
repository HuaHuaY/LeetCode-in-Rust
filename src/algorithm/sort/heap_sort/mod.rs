pub struct Solution;

impl Solution {
    fn build_heap(nums: &mut [i32]) {
        for i in (0..=nums.len() / 2).rev() {
            Solution::adjust_heap(nums, nums.len(), i);
        }
    }

    fn adjust_heap(nums: &mut [i32], len: usize, index: usize) {
        let mut max = index;
        if index * 2 < len && nums[index * 2] > nums[max] {
            max = index * 2;
        }
        if index * 2 + 1 < len && nums[index * 2 + 1] > nums[max] {
            max = index * 2 + 1;
        }

        if index != max {
            nums.swap(index, max);
            Solution::adjust_heap(nums, len, max);
        }
    }

    pub fn heap_sort(nums: &mut [i32]) {
        let mut len = nums.len();
        Solution::build_heap(nums);
        while len > 0 {
            nums.swap(0, len - 1);
            Solution::adjust_heap(nums, len - 1, 0);
            len -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = [2, 4, 1, 3, 5];
        Solution::heap_sort(&mut nums);
        assert_eq!(nums, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test2() {
        let mut nums = [1, 2, 0, 0, 2];
        Solution::heap_sort(&mut nums);
        assert_eq!(nums, [0, 0, 1, 2, 2]);
    }

    #[test]
    fn test3() {
        let mut nums: [i32; 0] = [];
        Solution::heap_sort(&mut nums);
        assert_eq!(nums, []);
    }

    #[test]
    fn test4() {
        let mut nums = [0];
        Solution::heap_sort(&mut nums);
        assert_eq!(nums, [0]);
    }
}
