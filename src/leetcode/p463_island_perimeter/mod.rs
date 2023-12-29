pub struct Solution;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        for (i, row) in grid.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if *item == 1 {
                    result += 4;
                    if i > 0 && grid[i - 1][j] == 1 {
                        result -= 2;
                    }
                    if j > 0 && grid[i][j - 1] == 1 {
                        result -= 2;
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
            Solution::island_perimeter(
                [[0, 1, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [1, 1, 0, 0]].to_vec_vec()
            ),
            16
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::island_perimeter([[1]].to_vec_vec()), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::island_perimeter([[1, 0]].to_vec_vec()), 4);
    }
}
