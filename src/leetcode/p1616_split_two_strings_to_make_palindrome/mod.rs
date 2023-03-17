pub struct Solution;

impl Solution {
    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        fn check(a: &[u8], b: &[u8]) -> bool {
            fn check_inner(a: &[u8]) -> bool {
                let mut i = 0;
                let mut j = a.len() - 1;
                while i < j && a[i] == a[j] {
                    i += 1;
                    j -= 1;
                }
                i >= j
            }
            let mut i = 0;
            let mut j = a.len() - 1;
            while i < j && a[i] == b[j] {
                i += 1;
                j -= 1;
            }
            if i >= j {
                true
            } else {
                check_inner(&a[i..=j]) || check_inner(&b[i..=j])
            }
        }

        let a = a.as_bytes();
        let b = b.as_bytes();
        check(a, b) || check(b, a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::check_palindrome_formation(
            "x".to_string(),
            "y".to_string()
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::check_palindrome_formation(
            "xbdef".to_string(),
            "xecab".to_string()
        ));
    }

    #[test]
    fn test3() {
        assert!(Solution::check_palindrome_formation(
            "ulacfd".to_string(),
            "jizalu".to_string()
        ));
    }
}
