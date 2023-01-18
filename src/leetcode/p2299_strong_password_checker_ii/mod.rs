pub struct Solution;

impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        if password.len() < 8 {
            return false;
        }

        let mut lowercase = false;
        let mut uppercase = false;
        let mut digit = false;
        let mut character = false;

        let mut last_byte = 0;
        for c in password.bytes() {
            if last_byte == c {
                return false;
            }
            last_byte = c;

            match c {
                b'a'..=b'z' => lowercase = true,
                b'A'..=b'Z' => uppercase = true,
                b'0'..=b'9' => digit = true,
                b'!' | b'@' | b'#' | b'$' | b'%' | b'^' | b'&' | b'*' | b'(' | b')' | b'-'
                | b'+' => character = true,
                _ => unreachable!(),
            }
        }
        lowercase && uppercase && digit && character
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::strong_password_checker_ii(
            "IloveLe3tcode!".to_string()
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::strong_password_checker_ii(
            "Me+You--IsMyDream".to_string()
        ));
    }

    #[test]
    fn test3() {
        assert!(!Solution::strong_password_checker_ii("1aB!".to_string()));
    }
}
