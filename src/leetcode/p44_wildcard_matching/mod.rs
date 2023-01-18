pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        let p = p.chars().collect::<Vec<_>>();
        let m = s.len();
        let n = p.len();

        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;
        for j in 1..=n {
            if p[j - 1] == '*' {
                dp[0][j] = dp[0][j - 1];
            } else {
                break;
            }
        }
        for i in 1..=m {
            for j in 1..=n {
                if p[j - 1] == '?' || p[j - 1] == s[i - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else if p[j - 1] == '*' {
                    dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
                }
            }
        }
        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(!Solution::is_match("aa".to_string(), "a".to_string()));
    }

    #[test]
    fn test2() {
        assert!(Solution::is_match("aa".to_string(), "*".to_string()));
    }

    #[test]
    fn test3() {
        assert!(!Solution::is_match("cb".to_string(), "?a".to_string()));
    }
}
