pub struct Solution;

impl Solution {
    fn binary_search(nums: &[i32], target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        while left <= right {
            let mid = (right - left) / 2 + left;
            if nums[mid as usize] >= target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        left
    }

    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let left = Solution::binary_search(&nums, target) as usize;
        let right = Solution::binary_search(&nums, target + 1) - 1;
        if left < nums.len() && nums[left] == target {
            vec![left as i32, right]
        } else {
            vec![-1, -1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8), [3, 4]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6), [-1, -1]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::search_range(vec![], 0), [-1, -1]);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::search_range(vec![1], 0), [-1, -1]);
    }
}
