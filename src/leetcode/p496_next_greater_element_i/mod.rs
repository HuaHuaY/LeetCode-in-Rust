pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut monotonic_stack = Vec::with_capacity(nums2.len());
        let mut results_map = HashMap::new();

        for i in nums2 {
            while !monotonic_stack.is_empty() && i > *monotonic_stack.last().unwrap() {
                results_map.insert(monotonic_stack.pop().unwrap(), i);
            }
            monotonic_stack.push(i);
        }
        while !monotonic_stack.is_empty() {
            results_map.insert(monotonic_stack.pop().unwrap(), -1);
        }

        let mut result = Vec::with_capacity(nums1.len());
        for i in nums1 {
            result.push(*results_map.get(&i).unwrap());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
            [-1, 3, -1]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
            [3, -1]
        );
    }
}
