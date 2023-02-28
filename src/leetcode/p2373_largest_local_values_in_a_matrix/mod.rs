pub struct Solution;

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let mut result = vec![vec![0; n - 2]; n - 2];
        for i in 0..n - 2 {
            for j in 0..n - 2 {
                result[i][j] = grid[i][j];
                for k in 0..3 {
                    for l in 0..3 {
                        result[i][j] = result[i][j].max(grid[i + k][j + l]);
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::largest_local(
                [[9, 9, 8, 1], [5, 6, 2, 6], [8, 2, 6, 4], [6, 2, 2, 2]].to_vec_vec()
            ),
            [[9, 9], [8, 6]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::largest_local(
                [
                    [1, 1, 1, 1, 1],
                    [1, 1, 1, 1, 1],
                    [1, 1, 2, 1, 1],
                    [1, 1, 1, 1, 1],
                    [1, 1, 1, 1, 1]
                ]
                .to_vec_vec()
            ),
            [[2, 2, 2], [2, 2, 2], [2, 2, 2]]
        );
    }
}
