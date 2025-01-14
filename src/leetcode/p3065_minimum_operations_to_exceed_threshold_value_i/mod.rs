pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.into_iter().filter(|n| *n < k).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_operations([2, 11, 10, 1, 3].to_vec(), 10), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_operations([1, 1, 2, 4, 9].to_vec(), 1), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::min_operations([1, 1, 2, 4, 9].to_vec(), 9), 4);
    }
}
