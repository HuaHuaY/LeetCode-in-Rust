pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        Self::foo2(nums)
    }

    fn foo1(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];
        let mut answer = 1;

        for i in 1..nums.len() {
            for j in 0..i {
                if nums[j] < nums[i] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            answer = answer.max(dp[i]);
        }
        answer
    }

    fn foo2(nums: Vec<i32>) -> i32 {
        let mut array = vec![0; nums.len() + 1];
        let mut length = 1;

        array[1] = nums[0];
        for i in nums.into_iter().skip(1) {
            if i > array[length] {
                length += 1;
                array[length] = i;
            } else {
                let mut left = 1;
                let mut right = length;
                let mut mid;
                while left <= right {
                    mid = (right - left) / 2 + left;
                    if i > array[mid] {
                        left = mid + 1;
                    } else {
                        right = mid - 1;
                    }
                }
                array[left] = i;
            }
        }
        length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    }
}
