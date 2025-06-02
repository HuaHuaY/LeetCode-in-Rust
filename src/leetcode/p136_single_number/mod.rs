pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, x| acc ^ x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::single_number([2, 2, 1].to_vec()), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::single_number([4, 1, 2, 1, 2].to_vec()), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::single_number([1].to_vec()), 1);
    }
}
