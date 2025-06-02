pub struct Solution;

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        if n > 3 * limit {
            return 0;
        }
        let cal = |n| {
            if n >= 2 {
                n as i64 * (n as i64 - 1) / 2
            } else {
                0
            }
        };
        cal(n + 2) - 3 * cal(n - limit + 1) + 3 * cal(n - 2 * limit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::distribute_candies(5, 2), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::distribute_candies(3, 3), 10);
    }
}
