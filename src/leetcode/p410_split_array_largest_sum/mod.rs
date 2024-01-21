pub struct Solution;

impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        fn is_valid(nums: &[i32], k: i32, m: i32) -> bool {
            let mut sum = nums[0];
            let mut cnt = 1;
            for num in &nums[1..] {
                sum += num;
                if sum > m {
                    cnt += 1;
                    sum = *num;
                }
            }
            cnt <= k
        }

        let mut left = *nums.iter().max().unwrap();
        let mut right = nums.iter().sum::<i32>();
        let mut mid;
        while left <= right {
            mid = left + (right - left) / 2;
            if is_valid(&nums, k, mid) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::split_array([7, 2, 5, 10, 8].to_vec(), 2), 18);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::split_array([1, 2, 3, 4, 5].to_vec(), 2), 9);
    }
}
