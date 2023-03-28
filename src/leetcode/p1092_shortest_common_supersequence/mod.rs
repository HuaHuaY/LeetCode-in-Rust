pub struct Solution;

impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        fn dfs(dp: &mut [Vec<i32>], i: usize, j: usize, str1: &[u8], str2: &[u8]) -> i32 {
            if dp[i][j] != -1 {
                return dp[i][j];
            }
            dp[i][j] = match (i == str1.len(), j == str2.len()) {
                (true, true) => 0,
                (true, false) => (str2.len() - j) as i32,
                (false, true) => (str1.len() - i) as i32,
                (false, false) => {
                    if str1[i] == str2[j] {
                        dfs(dp, i + 1, j + 1, str1, str2) + 1
                    } else {
                        dfs(dp, i + 1, j, str1, str2).min(dfs(dp, i, j + 1, str1, str2)) + 1
                    }
                }
            };
            dp[i][j]
        }
        let mut dp = vec![vec![-1; str2.len() + 1]; str1.len() + 1];
        let str1 = str1.as_bytes();
        let str2 = str2.as_bytes();
        let len = dfs(&mut dp, 0, 0, str1, str2);
        let mut result = String::with_capacity(len as usize);
        let mut i = 0;
        let mut j = 0;
        while i < str1.len() && j < str2.len() {
            if str1[i] == str2[j] {
                result.push(str1[i] as char);
                i += 1;
                j += 1;
            } else if dp[i][j] == dp[i + 1][j] + 1 {
                result.push(str1[i] as char);
                i += 1;
            } else {
                result.push(str2[j] as char);
                j += 1;
            }
        }
        result.push_str(std::str::from_utf8(&str1[i..str1.len()]).unwrap());
        result.push_str(std::str::from_utf8(&str2[j..str2.len()]).unwrap());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::shortest_common_supersequence("abac".to_string(), "cab".to_string()),
            "cabac"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::shortest_common_supersequence("aaaaaaaa".to_string(), "aaaaaaaa".to_string()),
            "aaaaaaaa"
        );
    }
}
