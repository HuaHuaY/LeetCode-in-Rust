pub struct Solution;

impl Solution {
    pub fn minimum_possible_sum(n: i32, target: i32) -> i32 {
        let n = n as i64;
        let target = target as i64;
        let left = (target / 2).min(n);
        let right = n - left;
        ((((left + 1) * left / 2 % 1_000_000_007)
            + ((target + right - 1 + target) * right / 2) % 1_000_000_007)
            % 1_000_000_007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_possible_sum(2, 3), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::minimum_possible_sum(3, 3), 8);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::minimum_possible_sum(1, 1), 1);
    }
}
