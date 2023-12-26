pub struct Solution;

impl Solution {
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        fn is_single_row_compliant(seats: &[Vec<char>], i: usize, n: usize) -> bool {
            if n & (n << 1) != 0 {
                return false;
            }
            for (j, seat) in seats[i].iter().enumerate() {
                if (1 << j) & n != 0 && seat == &'#' {
                    return false;
                }
            }
            true
        }

        fn is_cross_row_compliant(seats: &[Vec<char>], i: usize, n: usize, n_after: usize) -> bool {
            if n & (n_after << 1) != 0 || n & (n_after >> 1) != 0 {
                return false;
            }
            is_single_row_compliant(seats, i, n)
        }

        fn dp(cache: &mut [Vec<i32>], seats: &[Vec<char>], i: usize, n: usize) -> i32 {
            if cache[i][n] != -1 {
                return cache[i][n];
            }
            let mut max = n.count_ones() as i32;
            if i != 0 {
                for j in 0..(1 << seats[0].len()) {
                    if is_cross_row_compliant(seats, i - 1, j, n) {
                        max = max.max(dp(cache, seats, i - 1, j) + n.count_ones() as i32);
                    }
                }
            }
            cache[i][n] = max;
            max
        }

        let mut max = 0;
        for i in 0..(1 << seats[0].len()) {
            let len = seats.len();
            if is_single_row_compliant(&seats, len - 1, i) {
                max = max.max(dp(
                    &mut vec![vec![-1; 1 << seats[0].len()]; len],
                    &seats,
                    len - 1,
                    i,
                ));
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVecChar;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_students(
                [
                    ["#", ".", "#", "#", ".", "#"],
                    [".", "#", "#", "#", "#", "."],
                    ["#", ".", "#", "#", ".", "#"]
                ]
                .to_vec_vec_char()
            ),
            4
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_students(
                [[".", "#"], ["#", "#"], ["#", "."], ["#", "#"], [".", "#"]].to_vec_vec_char()
            ),
            3
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::max_students(
                [
                    ["#", ".", ".", ".", "#"],
                    [".", "#", ".", "#", "."],
                    [".", ".", "#", ".", "."],
                    [".", "#", ".", "#", "."],
                    ["#", ".", ".", ".", "#"]
                ]
                .to_vec_vec_char()
            ),
            10
        );
    }
}
