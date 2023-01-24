pub struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        fn dfs(
            n: usize,
            row: usize,
            col_check: i32,
            left_check: i32,
            right_check: i32,
            res: &mut i32,
        ) {
            if row == n {
                *res += 1;
                return;
            }

            let mut available = ((1 << n) - 1) & (!(col_check | left_check | right_check));
            while available != 0 {
                let pos = available & (-available);
                available = available & (available - 1);
                dfs(
                    n,
                    row + 1,
                    col_check | pos,
                    (left_check | pos) >> 1,
                    (right_check | pos) << 1,
                    res,
                );
            }
        }

        let mut res = 0;
        dfs(n as usize, 0, 0, 0, 0, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::total_n_queens(4), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::total_n_queens(1), 1);
    }
}
