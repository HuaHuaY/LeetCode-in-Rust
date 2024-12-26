pub struct Solution;

impl Solution {
    pub fn is_substring_present(s: String) -> bool {
        let mut visited = [0; 26];
        for chunk in s.as_bytes().windows(2) {
            let (a, b) = (chunk[0], chunk[1]);
            visited[(a - b'a') as usize] |= 1 << (b - b'a');
            if visited[(b - b'a') as usize] & (1 << (a - b'a')) != 0 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_substring_present("leetcode".to_string()));
    }

    #[test]
    fn test2() {
        assert!(Solution::is_substring_present("abcba".to_string()));
    }

    #[test]
    fn test3() {
        assert!(!Solution::is_substring_present("abcd".to_string()));
    }
}
