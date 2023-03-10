pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        fn dfs(
            cache: &mut HashSet<(usize, usize)>,
            s1: &[u8],
            s2: &[u8],
            s3: &[u8],
            i: usize,
            j: usize,
        ) -> bool {
            if cache.contains(&(i, j)) {
                return false;
            }
            if i == s1.len() && j == s2.len() {
                return true;
            }
            if i < s1.len() && s1[i] == s3[i + j] && dfs(cache, s1, s2, s3, i + 1, j) {
                return true;
            }
            if j < s2.len() && s2[j] == s3[i + j] && dfs(cache, s1, s2, s3, i, j + 1) {
                return true;
            }
            cache.insert((i, j));
            false
        }

        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        dfs(
            &mut HashSet::new(),
            s1.as_bytes(),
            s2.as_bytes(),
            s3.as_bytes(),
            0,
            0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbcbcac".to_string()
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbbaccc".to_string()
        ));
    }

    #[test]
    fn test3() {
        assert!(Solution::is_interleave(
            "".to_string(),
            "".to_string(),
            "".to_string()
        ));
    }
}
