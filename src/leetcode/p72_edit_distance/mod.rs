pub struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.chars().collect::<Vec<_>>();
        let word2 = word2.chars().collect::<Vec<_>>();
        let mut dp = vec![vec![0; word2.len() + 1]; word1.len() + 1];
        for i in 0..word1.len() + 1 {
            for j in 0..word2.len() + 1 {
                if i == 0 {
                    dp[i][j] = j;
                } else if j == 0 {
                    dp[i][j] = i;
                } else {
                    dp[i][j] = if word1[i - 1] == word2[j - 1] {
                        dp[i - 1][j - 1]
                    } else {
                        dp[i - 1][j - 1] + 1
                    };
                    dp[i][j] = dp[i][j].min(dp[i - 1][j] + 1).min(dp[i][j - 1] + 1);
                }
            }
        }
        dp[word1.len()][word2.len()] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_distance("horse".to_string(), "ros".to_string()),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_distance("intention".to_string(), "execution".to_string()),
            5
        );
    }
}
