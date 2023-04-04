pub struct Solution;

impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let min = a.min(b);
        let mut i = 1;
        let mut result = 0;
        while i * i <= min {
            if min % i == 0 {
                if a % i == 0 && b % i == 0 {
                    result += 1;
                }
                if i * i != min && a % (min / i) == 0 && b % (min / i) == 0 {
                    result += 1;
                }
            }
            i += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::common_factors(12, 6), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::common_factors(25, 30), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::common_factors(850, 442), 4);
    }
}
