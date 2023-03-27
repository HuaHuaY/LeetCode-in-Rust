pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String, t: String) -> i32 {
        let mut dp = vec![vec![vec![0; 2]; t.len()]; s.len()];
        let s = s.as_bytes();
        let t = t.as_bytes();
        for (i, s) in s.iter().enumerate() {
            for (j, t) in t.iter().enumerate() {
                if *s == *t {
                    dp[i][j][0] = 1;
                    if i > 0 && j > 0 {
                        dp[i][j][0] += dp[i - 1][j - 1][0];
                        dp[i][j][1] += dp[i - 1][j - 1][1];
                    }
                } else {
                    dp[i][j][1] = 1;
                    if i > 0 && j > 0 {
                        dp[i][j][1] += dp[i - 1][j - 1][0];
                    }
                }
            }
        }
        dp.into_iter()
            .flat_map(|row| row.into_iter().map(|line| line[1]))
            .sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_substrings("aba".to_string(), "baba".to_string()),
            6
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::count_substrings("ab".to_string(), "bb".to_string()),
            3
        );
    }
}
