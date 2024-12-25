pub struct Solution;

use std::cmp::Reverse;

impl Solution {
    pub fn minimum_cost(
        m: i32,
        n: i32,
        mut horizontal_cut: Vec<i32>,
        mut vertical_cut: Vec<i32>,
    ) -> i32 {
        horizontal_cut.sort_unstable_by_key(|x| Reverse(*x));
        vertical_cut.sort_unstable_by_key(|x| Reverse(*x));

        let mut result = 0;
        let mut i = 0;
        let mut j = 0;
        let mut ix = 1;
        let mut jx = 1;
        while i < m - 1 && j < n - 1 {
            let left_more = if horizontal_cut[i as usize] > vertical_cut[j as usize] {
                1
            } else {
                0
            };
            let right_more = 1 - left_more;
            result += left_more * horizontal_cut[i as usize] * ix;
            result += right_more * vertical_cut[j as usize] * jx;
            jx += left_more;
            ix += right_more;
            i += left_more;
            j += right_more;
        }
        result += horizontal_cut[i as usize..]
            .iter()
            .map(|x| *x * ix)
            .sum::<i32>();
        result += vertical_cut[j as usize..]
            .iter()
            .map(|x| *x * jx)
            .sum::<i32>();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::minimum_cost(3, 2, [1, 3].to_vec(), [5].to_vec()),
            13
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::minimum_cost(2, 2, [7].to_vec(), [4].to_vec()), 15);
    }
}
