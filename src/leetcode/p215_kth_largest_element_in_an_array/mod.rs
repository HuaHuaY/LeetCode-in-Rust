pub struct Solution;

use std::cmp::Reverse;

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        *nums
            .select_nth_unstable_by_key(k as usize - 1, |&i| Reverse(i))
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_kth_largest([3, 2, 1, 5, 6, 4].to_vec(), 2),
            5
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_kth_largest([3, 2, 3, 1, 2, 4, 5, 5, 6].to_vec(), 4),
            4
        );
    }
}
