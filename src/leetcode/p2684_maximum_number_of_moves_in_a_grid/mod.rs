pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut pre = (0..m).collect::<HashSet<_>>();
        let mut now = HashSet::with_capacity(m);
        for j in 0..n - 1 {
            for &i in &pre {
                if grid[i][j] < grid[i][j + 1] {
                    now.insert(i);
                }
                if i >= 1 && grid[i][j] < grid[i - 1][j + 1] {
                    now.insert(i - 1);
                }
                if i + 1 < m && grid[i][j] < grid[i + 1][j + 1] {
                    now.insert(i + 1);
                }
            }
            if now.is_empty() {
                return j as i32;
            }
            std::mem::swap(&mut pre, &mut now);
            now.clear();
        }
        n as i32 - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_moves(
                [[2, 4, 3, 5], [5, 4, 9, 3], [3, 4, 2, 11], [10, 9, 13, 15]].to_vec_vec()
            ),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_moves([[3, 2, 4], [2, 1, 9], [1, 1, 7]].to_vec_vec()),
            0
        );
    }
}
