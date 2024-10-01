pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut one_line_zero = false;
        let m = matrix.len();
        let n = matrix[0].len();

        for j in 0..n {
            if matrix[0][j] == 0 {
                one_line_zero = true;
                break;
            }
        }

        for i in 1..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        for i in 1..m {
            for j in (0..n).rev() {
                matrix[i][j] = if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    0
                } else {
                    matrix[i][j]
                }
            }
        }

        if one_line_zero {
            for j in 0..n {
                matrix[0][j] = 0;
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
        let mut test = [[1, 1, 1], [1, 0, 1], [1, 1, 1]].to_vec_vec();
        Solution::set_zeroes(&mut test);
        assert_eq!(test, [[1, 0, 1], [0, 0, 0], [1, 0, 1]]);
    }

    #[test]
    fn test2() {
        let mut test = [[0, 1, 2, 0], [3, 4, 5, 2], [1, 3, 1, 5]].to_vec_vec();
        Solution::set_zeroes(&mut test);
        assert_eq!(test, [[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]]);
    }
}
