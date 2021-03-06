struct Solution {}

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let length = nums.len();
        let mut monotonic_stack = Vec::with_capacity(length);
        let mut result = vec![-1; length];

        for i in 0..2 * length as i32 - 1 {
            while monotonic_stack.len() != 0
                && nums[i as usize % length] > nums[*monotonic_stack.last().unwrap()]
            {
                result[monotonic_stack.pop().unwrap()] = nums[i as usize % length];
            }
            monotonic_stack.push(i as usize % length);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::next_greater_elements(vec![1, 2, 1]), [2, -1, 2]);
    }
}
