pub struct Solution;

impl Solution {
    pub fn max_array_value(nums: Vec<i32>) -> i64 {
        nums.into_iter()
            .rev()
            .map(i64::from)
            .reduce(|a, b| if a >= b { a + b } else { b })
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_array_value([2, 3, 7, 9, 3].to_vec()), 21);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_array_value([5, 3, 3].to_vec()), 11);
    }
}
