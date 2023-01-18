pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = [[false; 9]; 9];
        let mut cols = [[false; 9]; 9];
        let mut boxs = [[[false; 9]; 3]; 3];
        for (i, row) in rows.iter_mut().enumerate() {
            for (j, col) in cols.iter_mut().enumerate() {
                if board[i][j] == '.' {
                    continue;
                }
                let num = board[i][j] as usize - '1' as usize;
                if row[num] || col[num] || boxs[i / 3][j / 3][num] {
                    return false;
                }
                row[num] = true;
                col[num] = true;
                boxs[i / 3][j / 3][num] = true;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::common::ToVecVec;

    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_valid_sudoku(
            [
                ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                ['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]
            .to_vec_vec()
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_valid_sudoku(
            [
                ['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                ['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]
            .to_vec_vec()
        ));
    }
}
