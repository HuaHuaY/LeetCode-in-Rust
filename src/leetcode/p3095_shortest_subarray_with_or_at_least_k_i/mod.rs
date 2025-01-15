pub struct Solution;

impl Solution {
    pub fn minimum_subarray_length(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut result = usize::MAX;
        for i in 0..nums.len() {
            if nums[i] >= k {
                return 1;
            }
            for j in (0..i).rev() {
                if (nums[i] | nums[j]) >= k {
                    result = result.min(i - j + 1);
                }
                if (nums[i] | nums[j]) == nums[j] {
                    break;
                }
                nums[j] |= nums[i];
            }
        }
        if result == usize::MAX {
            -1
        } else {
            result as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_subarray_length([1, 2, 3].to_vec(), 2), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::minimum_subarray_length([2, 1, 8].to_vec(), 10), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::minimum_subarray_length([1, 2].to_vec(), 0), 1);
    }
}
