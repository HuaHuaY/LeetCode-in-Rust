pub struct Solution;

impl Solution {
    pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
        let mut max_len = 0;
        let mut dp = vec![vec![[0; 2]; grid[0].len()]; grid.len()];
        for (i, row) in grid.into_iter().enumerate() {
            for (j, col) in row.into_iter().enumerate() {
                if col == 1 {
                    if i == 0 {
                        dp[i][j][1] = 1;
                    } else {
                        dp[i][j][1] = dp[i - 1][j][1] + 1;
                    }
                    if j == 0 {
                        dp[i][j][0] = 1;
                    } else {
                        dp[i][j][0] = dp[i][j - 1][0] + 1;
                    }
                    let len = std::cmp::min(dp[i][j][0], dp[i][j][1]);
                    for k in (1..=len).rev() {
                        if dp[i + 1 - k][j][0] >= k && dp[i][j + 1 - k][1] >= k {
                            max_len = std::cmp::max(max_len, k as i32);
                            break;
                        }
                    }
                }
            }
        }
        max_len * max_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::largest1_bordered_square([[1, 1, 1], [1, 0, 1], [1, 1, 1]].to_vec_vec()),
            9
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::largest1_bordered_square([[1, 1, 0, 0]].to_vec_vec()),
            1
        );
    }
}
