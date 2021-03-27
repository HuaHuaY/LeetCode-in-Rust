pub struct Solution {}

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut increase_stack = Vec::with_capacity(nums.len());
        let mut smallest_pre = Vec::with_capacity(nums.len());
        increase_stack.push(0);
        smallest_pre.push(nums[0]);
        for i in 1..nums.len() {
            while !increase_stack.is_empty() && nums[i] >= nums[*increase_stack.last().unwrap()] {
                increase_stack.pop();
            }

            if !increase_stack.is_empty() && nums[i] > smallest_pre[*increase_stack.last().unwrap()]
            {
                return true;
            }

            increase_stack.push(i);
            let tmp = *smallest_pre.last().unwrap();
            smallest_pre.push(tmp.min(nums[i]));
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::find132pattern(vec![1, 2, 3, 4]), false);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find132pattern(vec![3, 1, 4, 2]), true);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::find132pattern(vec![-1, 3, 2, 0]), true);
    }
}
