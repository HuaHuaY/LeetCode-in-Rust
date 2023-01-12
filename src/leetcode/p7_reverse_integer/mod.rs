pub struct Solution;

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut result = 0;
        while x != 0 {
            if result > i32::max_value() / 10 || result < i32::min_value() / 10 {
                return 0;
            }

            result = result * 10 + x % 10;
            x /= 10;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::reverse(120), 21);
    }
}
