pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut x = 0;
        let mut y = matrix[0].len();
        while x < matrix.len() && y != 0 {
            match matrix[x][y - 1].cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Less => x += 1,
                Ordering::Greater => y -= 1,
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert!(Solution::search_matrix(
            [
                [1, 4, 7, 11, 15],
                [2, 5, 8, 12, 19],
                [3, 6, 9, 16, 22],
                [10, 13, 14, 17, 24],
                [18, 21, 23, 26, 30]
            ]
            .to_vec_vec(),
            5
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::search_matrix(
            [
                [1, 4, 7, 11, 15],
                [2, 5, 8, 12, 19],
                [3, 6, 9, 16, 22],
                [10, 13, 14, 17, 24],
                [18, 21, 23, 26, 30]
            ]
            .to_vec_vec(),
            20
        ));
    }

    #[test]
    fn test3() {
        assert!(Solution::search_matrix([[-1, 3]].to_vec_vec(), 3));
    }
}
