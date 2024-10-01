pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let length = matrix.len();
        for i in 0..length / 2 {
            matrix.swap(i, length - 1 - i);
        }
        for i in 0..length {
            for j in 0..i {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        let mut matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]].to_vec_vec();
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, [[7, 4, 1], [8, 5, 2], [9, 6, 3]]);
    }

    #[test]
    fn test2() {
        let mut matrix = [
            [5, 1, 9, 11],
            [2, 4, 8, 10],
            [13, 3, 6, 7],
            [15, 14, 12, 16],
        ]
        .to_vec_vec();
        Solution::rotate(&mut matrix);
        assert_eq!(
            matrix,
            [
                [15, 13, 2, 5],
                [14, 3, 4, 1],
                [12, 6, 8, 9],
                [16, 7, 10, 11]
            ]
        );
    }
}
