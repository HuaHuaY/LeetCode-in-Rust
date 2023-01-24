pub struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        fn dfs(
            queens: &mut [usize],
            row: usize,
            col_check: i32,
            left_check: i32,
            right_check: i32,
            res: &mut Vec<Vec<String>>,
        ) {
            if row == queens.len() {
                res.push(
                    queens
                        .iter()
                        .map(|pos| {
                            let mut v = vec!['.'; queens.len()];
                            v[*pos] = 'Q';
                            v.into_iter().collect()
                        })
                        .collect(),
                );
                return;
            }

            let mut available =
                ((1 << queens.len()) - 1) & (!(col_check | left_check | right_check));
            while available != 0 {
                let pos = available & (-available);
                available = available & (available - 1);
                let column = pos.trailing_zeros();
                queens[row] = column as usize;
                dfs(
                    queens,
                    row + 1,
                    col_check | pos,
                    (left_check | pos) >> 1,
                    (right_check | pos) << 1,
                    res,
                );
                queens[row] = queens.len();
            }
        }

        let mut queens = vec![n as usize; n as usize];
        let mut res = vec![];
        dfs(&mut queens, 0, 0, 0, 0, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::solve_n_queens(4),
            [
                [".Q..", "...Q", "Q...", "..Q."],
                ["..Q.", "Q...", "...Q", ".Q.."]
            ]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::solve_n_queens(1), [["Q"]]);
    }
}
