pub struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        true // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::is_match(String::from("aa"), String::from("a")),
            false
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::is_match(String::from("aa"), String::from("a*")),
            true
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::is_match(String::from("ab"), String::from(".*")),
            true
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::is_match(String::from("aab"), String::from("c*a*b")),
            true
        );
    }
}
