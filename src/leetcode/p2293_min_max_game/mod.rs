pub struct Solution;

impl Solution {
    pub fn min_max_game(mut nums: Vec<i32>) -> i32 {
        let mut len = nums.len();
        while len != 1 {
            len >>= 1;
            for i in 0..len {
                nums[i] = if i & 1 == 0 {
                    nums[2 * i].min(nums[2 * i + 1])
                } else {
                    nums[2 * i].max(nums[2 * i + 1])
                };
            }
        }
        nums[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_max_game(vec![1, 3, 5, 2, 4, 8, 2, 2]), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_max_game(vec![3]), 3);
    }
}
