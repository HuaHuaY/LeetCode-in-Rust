pub struct Solution;

impl Solution {
    pub fn min_increments(n: i32, mut cost: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in (0..(n as usize - 1) / 2).rev() {
            let left = cost[2 * i + 1];
            let right = cost[2 * i + 2];
            result += (left - right).abs();
            cost[i] += left.max(right);
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
            Solution::min_increments(7, [1, 5, 2, 2, 3, 3, 1].to_vec()),
            6
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_increments(3, [5, 3, 3].to_vec()), 0);
    }
}
