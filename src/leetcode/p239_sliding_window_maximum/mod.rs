pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut result = Vec::with_capacity(nums.len() - k + 1);
        let mut windows = VecDeque::with_capacity(k);
        for i in 0..k {
            while let Some(&idx) = windows.back() {
                if nums[i] > nums[idx] {
                    windows.pop_back();
                } else {
                    break;
                }
            }
            windows.push_back(i);
        }
        result.push(nums[*windows.front().unwrap()]);
        for i in k..nums.len() {
            if *windows.front().unwrap() == i - k {
                windows.pop_front();
            }
            while let Some(&idx) = windows.back() {
                if nums[i] > nums[idx] {
                    windows.pop_back();
                } else {
                    break;
                }
            }
            windows.push_back(i);
            result.push(nums[*windows.front().unwrap()]);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_sliding_window([1, 3, -1, -3, 5, 3, 6, 7].to_vec(), 3),
            [3, 3, 5, 5, 6, 7]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_sliding_window([1].to_vec(), 1), [1]);
    }
}
