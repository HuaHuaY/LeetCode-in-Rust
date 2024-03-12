pub struct Solution;

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let count = s.bytes().filter(|&b| b == b'1').count();
        format!("{}{}1", "1".repeat(count - 1), "0".repeat(s.len() - count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::maximum_odd_binary_number("010".to_string()),
            "001"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::maximum_odd_binary_number("0101".to_string()),
            "1001"
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::maximum_odd_binary_number("0111".to_string()),
            "1101"
        );
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::maximum_odd_binary_number("11".to_string()), "11");
    }
}
