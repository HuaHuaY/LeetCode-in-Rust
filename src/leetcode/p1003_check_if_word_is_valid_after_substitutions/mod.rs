pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let s = s.as_bytes();
        let mut v = vec![];
        for b in s {
            let len = v.len();
            if len >= 2 && *b == b'c' && v[len - 1] == b'b' && v[len - 2] == b'a' {
                v.pop();
                v.pop();
                continue;
            }
            v.push(*b);
        }
        v.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_valid("aabcbc".to_string()));
    }

    #[test]
    fn test2() {
        assert!(Solution::is_valid("abcabcababcc".to_string()));
    }

    #[test]
    fn test3() {
        assert!(!Solution::is_valid("abccba".to_string()));
    }
}
