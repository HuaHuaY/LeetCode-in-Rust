pub struct Solution;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let mut is_palindrome_i_j = vec![vec![true; s.len()]; s.len()];
        let s: Vec<char> = s.chars().collect();

        for i in (0..s.len()).rev() {
            for j in i + 1..s.len() {
                is_palindrome_i_j[i][j] = (s[i] == s[j]) && is_palindrome_i_j[i + 1][j - 1];
            }
        }
        let mut dp = vec![0; s.len()];
        for i in 0..s.len() {
            if !is_palindrome_i_j[0][i] {
                dp[i] = i;
                for j in 0..i {
                    if is_palindrome_i_j[j + 1][i] {
                        dp[i] = dp[i].min(dp[j] + 1);
                    }
                }
            }
        }
        dp[s.len() - 1] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_cut("aab".to_string()), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_cut("a".to_string()), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::min_cut("ab".to_string()), 1);
    }
}
