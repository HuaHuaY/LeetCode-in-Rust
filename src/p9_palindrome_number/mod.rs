struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        x.to_string() == x.to_string().chars().rev().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::is_palindrome(121), true);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::is_palindrome(-121), false);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::is_palindrome(10), false);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::is_palindrome(-101), false);
    }
}
