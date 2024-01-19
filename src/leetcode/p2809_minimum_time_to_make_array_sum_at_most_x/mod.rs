pub struct Solution;

use std::mem::swap;

impl Solution {
    pub fn minimum_time(nums1: Vec<i32>, nums2: Vec<i32>, x: i32) -> i32 {
        let sum1 = nums1.iter().sum::<i32>();
        let sum2 = nums2.iter().sum::<i32>();

        let mut nums = nums2.into_iter().zip(nums1).collect::<Vec<(i32, i32)>>();
        nums.sort();

        let mut dp1 = vec![0; nums.len() + 1];
        let mut dp2 = dp1.clone();

        for (i, num) in nums.into_iter().enumerate() {
            for (j, d) in dp2.iter_mut().enumerate().skip(1).take(i + 1) {
                *d = dp1[j].max(dp1[j - 1] + j as i32 * num.0 + num.1);
            }
            swap(&mut dp1, &mut dp2);
        }

        dp1.into_iter()
            .enumerate()
            .skip_while(|(i, v)| sum1 + sum2 * *i as i32 - *v > x)
            .map(|(i, _)| i as i32)
            .next()
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::minimum_time([1, 2, 3].to_vec(), [1, 2, 3].to_vec(), 4),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::minimum_time([1, 2, 3].to_vec(), [3, 3, 3].to_vec(), 4),
            -1
        );
    }
}
