pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn max_result(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = nums.len().min(k as usize);
        let mut window = VecDeque::with_capacity(k);

        window.push_back(0);

        for i in 1..nums.len() {
            nums[i] += nums[window[0]];
            if window[0] + k == i {
                window.pop_front();
            }
            while let Some(&v) = window.back() {
                if nums[v] <= nums[i] {
                    window.pop_back();
                } else {
                    break;
                }
            }
            window.push_back(i);
        }

        nums[nums.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_result([1, -1, -2, 4, -7, 3].to_vec(), 2), 7);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_result([10, -5, -2, 4, 0, 3].to_vec(), 3), 17);
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::max_result([1, -5, -20, 4, -1, 3, -6, -3].to_vec(), 2),
            0
        );
    }
}
