pub struct Solution {}

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
        assert!(Solution::is_palindrome(121));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_palindrome(-121));
    }

    #[test]
    fn test3() {
        assert!(!Solution::is_palindrome(10));
    }

    #[test]
    fn test4() {
        assert!(!Solution::is_palindrome(-101));
    }
}
