pub struct Solution;

impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        fn max(nums: &[i32], first_len: usize, second_len: usize) -> i32 {
            let mut first_sum = 0;
            let mut second_sum = 0;
            let mut first_left_max = 0;
            let mut max = 0;
            for (i, num) in nums.iter().enumerate() {
                if i < first_len {
                    first_sum += num;
                } else if i < first_len + second_len - 1 {
                    second_sum += num;
                } else if i == first_len + second_len - 1 {
                    second_sum += num;
                    first_left_max = first_sum;
                    max = first_left_max + second_sum;
                } else {
                    first_sum += nums[i - second_len] - nums[i - first_len - second_len];
                    first_left_max = first_left_max.max(first_sum);
                    second_sum += num - nums[i - second_len];
                    max = max.max(first_left_max + second_sum);
                }
            }
            max
        }

        let first_len = first_len as usize;
        let second_len = second_len as usize;
        max(&nums, first_len, second_len).max(max(&nums, second_len, first_len))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_sum_two_no_overlap([0, 6, 5, 2, 2, 5, 1, 9, 4].to_vec(), 1, 2),
            20
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_sum_two_no_overlap([3, 8, 1, 3, 2, 1, 8, 9, 0].to_vec(), 3, 2),
            29
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::max_sum_two_no_overlap([2, 1, 5, 6, 0, 9, 5, 0, 3, 8].to_vec(), 4, 3),
            31
        );
    }
}
