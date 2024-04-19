pub struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let m = s.len();
        let n = t.len();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for d in dp.iter_mut() {
            d[0] = 1;
        }
        for i in 1..=m {
            for j in 1..=n {
                if s[i - 1] == t[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i - 1][j];
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
        assert_eq!(
            Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string()),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::num_distinct("babgbag".to_string(), "bag".to_string()),
            5
        );
    }
}
