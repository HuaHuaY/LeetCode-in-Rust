pub struct Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let n = matrix[0].len();
        let mut dp = vec![0i32; n];
        let mut dp_next = dp.clone();
        let mut result = 0;
        for (i, row) in matrix.into_iter().enumerate() {
            if i == 0 {
                for (j, c) in row.into_iter().enumerate() {
                    if c == '1' {
                        dp[j] = 1;
                        result = 1;
                    }
                }
            } else {
                for (j, c) in row.into_iter().enumerate() {
                    if c == '1' {
                        if j == 0 {
                            dp_next[j] = 1;
                            result = result.max(1);
                        } else {
                            dp_next[j] = dp_next[j - 1].min(dp[j - 1]).min(dp[j]) + 1;
                            result = result.max(dp_next[j] * dp_next[j]);
                        }
                    } else {
                        dp_next[j] = 0;
                    }
                }
                std::mem::swap(&mut dp, &mut dp_next);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVecChar;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::maximal_square(
                [
                    ["1", "0", "1", "0", "0"],
                    ["1", "0", "1", "1", "1"],
                    ["1", "1", "1", "1", "1"],
                    ["1", "0", "0", "1", "0"]
                ]
                .to_vec_vec_char()
            ),
            4
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::maximal_square([["0", "1"], ["1", "0"]].to_vec_vec_char()),
            1
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::maximal_square([["0"]].to_vec_vec_char()), 0);
    }
}
