pub struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut max = 0;
        let mut dp = vec![vec![(0, 0); matrix[0].len()]; matrix.len()];
        for (i, row) in matrix.into_iter().enumerate() {
            for (j, c) in row.into_iter().enumerate() {
                if c != '0' {
                    let x = 1 + if i >= 1 { dp[i - 1][j].0 } else { 0 };
                    let y = 1 + if j >= 1 { dp[i][j - 1].1 } else { 0 };
                    dp[i][j] = (x, y);
                    let mut min = usize::MAX;
                    for x in 1..=x {
                        min = min.min(dp[i + 1 - x][j].1);
                        if min == 0 {
                            break;
                        }
                        max = max.max(x * min);
                    }
                }
            }
        }
        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVecChar;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::maximal_rectangle(
                [
                    ["1", "0", "1", "0", "0"],
                    ["1", "0", "1", "1", "1"],
                    ["1", "1", "1", "1", "1"],
                    ["1", "0", "0", "1", "0"]
                ]
                .to_vec_vec_char()
            ),
            6
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::maximal_rectangle([["0"]].to_vec_vec_char()), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::maximal_rectangle([["1"]].to_vec_vec_char()), 1);
    }
}
