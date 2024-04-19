pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = vec![false; s.len() + 1];
        let set = word_dict
            .iter()
            .map(|s| s.as_str())
            .collect::<HashSet<&str>>();
        let max_length = set.iter().map(|s| s.len()).max().unwrap();
        dp[0] = true;
        for i in 1..=s.len() {
            for j in 0.max(1 - max_length as i32) as usize..i {
                if dp[j] && set.contains(&s[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[s.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecString;

    #[test]
    fn test1() {
        assert!(Solution::word_break(
            "leetcode".to_string(),
            ["leet", "code"].to_vec_string()
        ));
    }

    #[test]
    fn test2() {
        assert!(Solution::word_break(
            "applepenapple".to_string(),
            ["apple", "pen"].to_vec_string()
        ));
    }

    #[test]
    fn test3() {
        assert!(!Solution::word_break(
            "catsandog".to_string(),
            ["cats", "dog", "sand", "and", "cat"].to_vec_string()
        ));
    }
}
