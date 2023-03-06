pub struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        fn dfs(
            board: &[Vec<char>],
            visited: &mut [Vec<bool>],
            x: usize,
            y: usize,
            i: usize,
            chars: &[char],
        ) -> bool {
            if i == chars.len() - 1 {
                return true;
            }
            visited[x][y] = true;
            if x >= 1
                && !visited[x - 1][y]
                && board[x - 1][y] == chars[i + 1]
                && dfs(board, visited, x - 1, y, i + 1, chars)
            {
                return true;
            }
            if y >= 1
                && !visited[x][y - 1]
                && board[x][y - 1] == chars[i + 1]
                && dfs(board, visited, x, y - 1, i + 1, chars)
            {
                return true;
            }
            if x + 1 < board.len()
                && !visited[x + 1][y]
                && board[x + 1][y] == chars[i + 1]
                && dfs(board, visited, x + 1, y, i + 1, chars)
            {
                return true;
            }
            if y + 1 < board[0].len()
                && !visited[x][y + 1]
                && board[x][y + 1] == chars[i + 1]
                && dfs(board, visited, x, y + 1, i + 1, chars)
            {
                return true;
            }
            visited[x][y] = false;
            false
        }

        let mut visited = vec![vec![false; board[0].len()]; board.len()];
        let chars = word.chars().collect::<Vec<_>>();
        board
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(j, c)| (*c == chars[0]).then(|| (i, j)))
                    .collect::<Vec<_>>()
            })
            .any(|(i, j)| dfs(&board, &mut visited, i, j, 0, &chars))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVecChar;

    #[test]
    fn test1() {
        assert!(Solution::exist(
            [
                ["A", "B", "C", "E"],
                ["S", "F", "C", "S"],
                ["A", "D", "E", "E"]
            ]
            .to_vec_vec_char(),
            "ABCCED".to_string()
        ));
    }

    #[test]
    fn test2() {
        assert!(Solution::exist(
            [
                ["A", "B", "C", "E"],
                ["S", "F", "C", "S"],
                ["A", "D", "E", "E"]
            ]
            .to_vec_vec_char(),
            "SEE".to_string()
        ));
    }

    #[test]
    fn test3() {
        assert!(!Solution::exist(
            [
                ["A", "B", "C", "E"],
                ["S", "F", "C", "S"],
                ["A", "D", "E", "E"]
            ]
            .to_vec_vec_char(),
            "ABCB".to_string()
        ));
    }

    #[test]
    fn test4() {
        assert!(Solution::exist(
            [
                ["A", "B", "C", "E"],
                ["S", "F", "E", "S"],
                ["A", "D", "E", "E"]
            ]
            .to_vec_vec_char(),
            "ABCESEEEFS".to_string()
        ));
    }
}
