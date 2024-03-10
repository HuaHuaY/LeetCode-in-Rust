pub struct Solution;

impl Solution {
    pub fn k_sum(mut nums: Vec<i32>, k: i32) -> i64 {
        fn check(nums: &[i32], k: &mut i32, sum: i64) -> bool {
            if *k == 0 {
                return true;
            }
            if nums.is_empty() || sum < nums[0] as i64 {
                return false;
            }
            *k -= 1;
            check(&nums[1..], k, sum - nums[0] as i64) || check(&nums[1..], k, sum)
        }

        let mut sum = 0;
        for num in nums.iter_mut() {
            if *num > 0 {
                sum += *num as i64;
            } else {
                *num = -*num;
            }
        }
        let len = nums.len();
        let (i, &mut j, _) = nums.select_nth_unstable(len.min(k as usize) - 1);
        nums = i.to_vec();
        nums.push(j);
        nums.sort_unstable();

        let mut left = 0;
        let mut right = nums.iter().map(|i| *i as i64).sum::<i64>();
        while left <= right {
            let mid = left + (right - left) / 2;
            if check(&nums, &mut (k - 1), mid) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        sum - left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::k_sum([2, 4, -2].to_vec(), 5), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::k_sum([1, -2, 3, 4, -10, 12].to_vec(), 16), 10);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::k_sum([-1, 1].to_vec(), 1), 1);
    }
}
