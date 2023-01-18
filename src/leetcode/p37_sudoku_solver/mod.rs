pub struct Solution;

use std::collections::VecDeque;

#[allow(dead_code)]
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Solution::foo1(board)
    }

    fn foo1(board: &mut Vec<Vec<char>>) {
        fn flip(
            line: &mut [u16; 9],
            column: &mut [u16; 9],
            block: &mut [[u16; 3]; 3],
            i: usize,
            j: usize,
            num: u8,
        ) {
            line[i] ^= 1 << num;
            column[j] ^= 1 << num;
            block[i / 3][j / 3] ^= 1 << num;
        }

        fn dfs(
            board: &mut Vec<Vec<char>>,
            spaces: &mut VecDeque<(usize, usize)>,
            idx: usize,
            line: &mut [u16; 9],
            column: &mut [u16; 9],
            block: &mut [[u16; 3]; 3],
        ) -> bool {
            if idx == spaces.len() {
                return true;
            }
            let (i, j) = spaces[idx];
            let mut mask = (!(line[i] | column[j] | block[i / 3][j / 3])) & 0x1ff;
            while mask != 0 {
                let num = mask.trailing_zeros() as u8;
                mask &= mask - 1;
                board[i][j] = (num + b'1') as char;
                flip(line, column, block, i, j, num);
                if dfs(board, spaces, idx + 1, line, column, block) {
                    return true;
                }
                flip(line, column, block, i, j, num);
            }
            false
        }

        let mut spaces = VecDeque::new();
        let mut line = [0u16; 9];
        let mut column = [0u16; 9];
        let mut block = [[0u16; 3]; 3];
        for (i, row) in board.iter().enumerate().take(9) {
            for (j, c) in row.iter().enumerate().take(9) {
                if *c != '.' {
                    let num = *c as u8 - b'1';
                    flip(&mut line, &mut column, &mut block, i, j, num);
                }
            }
        }

        loop {
            let modified = false;
            for i in 0..9 {
                for j in 0..9 {
                    if board[i][j] != '.' {
                        continue;
                    }
                    let mask = (!(line[i] | column[j] | block[i / 3][j / 3])) & 0x1ff;
                    if mask.count_ones() == 1 {
                        let num = mask.trailing_zeros() as u8;
                        board[i][j] = (num + b'1') as char;
                        flip(&mut line, &mut column, &mut block, i, j, num);
                    }
                }
            }
            if !modified {
                break;
            }
        }

        for (i, row) in board.iter().enumerate().take(9) {
            for (j, c) in row.iter().enumerate().take(9) {
                if *c == '.' {
                    spaces.push_back((i, j));
                }
            }
        }

        dfs(board, &mut spaces, 0, &mut line, &mut column, &mut block);
    }

    fn foo2(board: &mut Vec<Vec<char>>) {
        fn is_valid(board: &[Vec<char>], row: usize, col: usize, c: char) -> bool {
            for i in 0..9 {
                if board[row][i] == c {
                    return false;
                }
                if board[i][col] == c {
                    return false;
                }
                if board[(row / 3) * 3 + i / 3][(col / 3) * 3 + i % 3] == c {
                    return false;
                }
            }
            true
        }

        fn dfs(board: &mut Vec<Vec<char>>, row: usize, col: usize) -> bool {
            if col == 9 {
                return dfs(board, row + 1, 0);
            }
            if row == 9 {
                return true;
            }
            if board[row][col] != '.' {
                return dfs(board, row, col + 1);
            }
            for i in 1..=9 {
                let c = (i as u8 + b'0') as char;
                if !is_valid(board, row, col, c) {
                    continue;
                }
                board[row][col] = c;
                if dfs(board, row, col + 1) {
                    return true;
                }
                board[row][col] = '.';
            }
            false
        }

        dfs(board, 0, 0);
    }
}

#[cfg(test)]
mod tests {
    use crate::common::ToVecVec;

    use super::*;

    #[test]
    fn test1() {
        let mut board = [
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]
        .to_vec_vec();
        Solution::solve_sudoku(&mut board);
        assert_eq!(
            board,
            [
                ['5', '3', '4', '6', '7', '8', '9', '1', '2'],
                ['6', '7', '2', '1', '9', '5', '3', '4', '8'],
                ['1', '9', '8', '3', '4', '2', '5', '6', '7'],
                ['8', '5', '9', '7', '6', '1', '4', '2', '3'],
                ['4', '2', '6', '8', '5', '3', '7', '9', '1'],
                ['7', '1', '3', '9', '2', '4', '8', '5', '6'],
                ['9', '6', '1', '5', '3', '7', '2', '8', '4'],
                ['2', '8', '7', '4', '1', '9', '6', '3', '5'],
                ['3', '4', '5', '2', '8', '6', '1', '7', '9']
            ]
        );
    }
}
