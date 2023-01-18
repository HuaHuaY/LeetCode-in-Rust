pub struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let length = nums.len();

        for i in 0..length {
            let mut num = nums[i];
            while num >= 1 && num <= length as i32 && num != i as i32 + 1 {
                if nums[num as usize - 1] != num {
                    nums.swap(i, num as usize - 1);
                    num = nums[i];
                } else {
                    break;
                }
            }
        }

        nums.into_iter()
            .enumerate()
            .find(|(idx, num)| *idx + 1 != *num as usize)
            .map(|(idx, _)| idx as i32 + 1)
            .unwrap_or(length as i32 + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }
}
