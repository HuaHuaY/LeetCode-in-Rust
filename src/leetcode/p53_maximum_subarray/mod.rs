pub struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
        let mut last = 0;
        for i in nums {
            last = i.max(i + last);
            result = result.max(last);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(6, Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]));
    }

    #[test]
    fn test2() {
        assert_eq!(1, Solution::max_sub_array(vec![1]));
    }

    #[test]
    fn test3() {
        assert_eq!(23, Solution::max_sub_array(vec![5,4,-1,7,8]));
    }
}
