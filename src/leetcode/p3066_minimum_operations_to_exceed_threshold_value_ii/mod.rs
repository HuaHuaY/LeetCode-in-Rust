pub struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = nums
            .into_iter()
            .map(i64::from)
            .map(Reverse)
            .collect::<BinaryHeap<_>>();
        let mut result = 0;
        while heap.peek().unwrap().0 < k as i64 {
            result += 1;
            let num1 = heap.pop().unwrap().0;
            let num2 = heap.pop().unwrap().0;
            heap.push(Reverse(num1 + num2 + num1.min(num2)));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_operations([2, 11, 10, 1, 3].to_vec(), 10), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_operations([1, 1, 2, 4, 9].to_vec(), 20), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::min_operations([999999999, 999999999, 999999999].to_vec(), 1000000000),
            2
        );
    }
}
