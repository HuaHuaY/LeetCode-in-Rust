pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn min_cost(nums: Vec<i32>, x: i32) -> i64 {
        let mut min = nums.clone();
        let mut result = i64::MAX;
        for i in 0..nums.len() {
            for (j, e) in min.iter_mut().enumerate() {
                *e = (*e).min(nums[(i + j) % nums.len()]);
            }
            result = result.min(min.iter().map(|e| *e as i64).sum::<i64>() + i as i64 * x as i64);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_cost([20, 1, 15].to_vec(), 5), 13);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_cost([1, 2, 3].to_vec(), 4), 6);
    }
}
