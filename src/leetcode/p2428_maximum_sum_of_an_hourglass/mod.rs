pub struct Solution;

impl Solution {
    pub fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        for i in 0..(grid.len() - 2) {
            let mut sum = grid[i][0]
                + grid[i][1]
                + grid[i][2]
                + grid[i + 1][1]
                + grid[i + 2][0]
                + grid[i + 2][1]
                + grid[i + 2][2];
            max = max.max(sum);
            for j in 1..(grid[0].len() - 2) {
                sum = sum - grid[i][j - 1] + grid[i][j + 2] - grid[i + 1][j] + grid[i + 1][j + 1]
                    - grid[i + 2][j - 1]
                    + grid[i + 2][j + 2];
                max = max.max(sum);
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_sum(
                [[6, 2, 1, 3], [4, 2, 1, 5], [9, 2, 8, 7], [4, 1, 2, 9]].to_vec_vec()
            ),
            30
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_sum([[1, 2, 3], [4, 5, 6], [7, 8, 9]].to_vec_vec()),
            35
        );
    }
}
