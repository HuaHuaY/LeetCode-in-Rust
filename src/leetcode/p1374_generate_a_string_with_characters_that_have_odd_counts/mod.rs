pub struct Solution {}

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        "a".repeat(n as usize - 1) + if n % 2 == 0 { "b" } else { "a" }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::generate_the_string(4), "aaab");
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::generate_the_string(2), "ab");
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::generate_the_string(7), "aaaaaaa");
    }
}
