pub struct Solution {}

impl Solution {
    pub fn bubble_sort(nums: &mut [i32]) {
        for i in 0..nums.len() {
            for j in 0..nums.len() - i - 1 {
                if nums[j] >= nums[j + 1] {
                    nums.swap(j, j + 1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = [2, 4, 1, 3, 5];
        Solution::bubble_sort(&mut nums);
        assert_eq!([1, 2, 3, 4, 5], nums);
    }

    #[test]
    fn test2() {
        let mut nums = [1, 2, 0, 0, 2];
        Solution::bubble_sort(&mut nums);
        assert_eq!([0, 0, 1, 2, 2], nums);
    }

    #[test]
    fn test3() {
        let mut nums: [i32;0] = [];
        Solution::bubble_sort(&mut nums);
        assert_eq!([] as [i32;0], nums);
    }

    #[test]
    fn test4() {
        let mut nums = [0];
        Solution::bubble_sort(&mut nums);
        assert_eq!([0], nums);
    }
}
