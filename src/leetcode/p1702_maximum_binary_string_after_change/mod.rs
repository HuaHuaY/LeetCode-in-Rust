pub struct Solution;

impl Solution {
    pub fn maximum_binary_string(binary: String) -> String {
        let n = binary.len();
        let Some(i) = binary.find(|b| b == '0') else {
            return binary;
        };
        let count = binary.chars().filter(|&b| b == '0').count();
        let mut chars = vec!['1'; n];
        chars[i + count - 1] = '0';
        chars.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::maximum_binary_string("000110".to_string()),
            "111011"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::maximum_binary_string("01".to_string()), "01");
    }
}
