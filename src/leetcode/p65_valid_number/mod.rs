pub struct Solution;

impl Solution {
    pub fn is_number(s: String) -> bool {
        fn is_integer(s: &[char]) -> bool {
            let len = s.len();
            let mut i = 0;
            if len > 1 && (s[0] == '+' || s[0] == '-') {
                i += 1;
            }
            if len - i == 0 {
                return false;
            }
            for c in s[i..].iter() {
                if !c.is_digit(10) {
                    return false;
                }
            }
            true
        }

        fn is_decimal(s: &[char]) -> bool {
            let len = s.len();
            let mut i = 0;
            if len > 1 && (s[0] == '+' || s[0] == '-') {
                i += 1;
            }

            let dot_split = s[i..].split(|e| *e == '.').collect::<Vec<_>>();
            if dot_split.len() != 2 || (dot_split[0].is_empty() && dot_split[1].is_empty()) {
                return false;
            }
            for c in dot_split[0] {
                if !c.is_digit(10) {
                    return false;
                }
            }
            for c in dot_split[1] {
                if !c.is_digit(10) {
                    return false;
                }
            }
            true
        }

        let chars = s.chars().collect::<Vec<_>>();
        let e_split = chars.split(|e| *e == 'e' || *e == 'E').collect::<Vec<_>>();
        match e_split.len() {
            1 => is_decimal(e_split[0]) || is_integer(e_split[0]),
            2 => (is_decimal(e_split[0]) || is_integer(e_split[0])) && is_integer(e_split[1]),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_number("0".to_string()));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_number("e".to_string()));
    }

    #[test]
    fn test3() {
        assert!(!Solution::is_number(".".to_string()));
    }

    #[test]
    fn test4() {
        assert!(Solution::is_number("+.8".to_string()));
    }
}
