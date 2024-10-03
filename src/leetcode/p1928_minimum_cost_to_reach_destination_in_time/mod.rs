pub struct Solution;

impl Solution {
    pub fn min_cost(max_time: i32, edges: Vec<Vec<i32>>, passing_fees: Vec<i32>) -> i32 {
        let len = passing_fees.len();
        let mut dp = vec![vec![i32::MAX; len]; max_time as usize + 1];
        dp[0][0] = passing_fees[0];

        for t in 1..max_time + 1 {
            for edge in &edges {
                let (i, j, cost) = (edge[0] as usize, edge[1] as usize, edge[2]);
                if cost <= t {
                    if dp[(t - cost) as usize][j] != i32::MAX {
                        dp[t as usize][i] =
                            dp[t as usize][i].min(dp[(t - cost) as usize][j] + passing_fees[i]);
                    }
                    if dp[(t - cost) as usize][i] != i32::MAX {
                        dp[t as usize][j] =
                            dp[t as usize][j].min(dp[(t - cost) as usize][i] + passing_fees[j]);
                    }
                }
            }
        }

        let result = (1..max_time + 1)
            .map(|t| dp[t as usize][len - 1])
            .min()
            .unwrap();
        if result == i32::MAX {
            -1
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_cost(
                30,
                [
                    [0, 1, 10],
                    [1, 2, 10],
                    [2, 5, 10],
                    [0, 3, 1],
                    [3, 4, 10],
                    [4, 5, 15]
                ]
                .to_vec_vec(),
                [5, 1, 2, 20, 20, 3].to_vec()
            ),
            11
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_cost(
                29,
                [
                    [0, 1, 10],
                    [1, 2, 10],
                    [2, 5, 10],
                    [0, 3, 1],
                    [3, 4, 10],
                    [4, 5, 15]
                ]
                .to_vec_vec(),
                [5, 1, 2, 20, 20, 3].to_vec()
            ),
            48
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::min_cost(
                25,
                [
                    [0, 1, 10],
                    [1, 2, 10],
                    [2, 5, 10],
                    [0, 3, 1],
                    [3, 4, 10],
                    [4, 5, 15]
                ]
                .to_vec_vec(),
                [5, 1, 2, 20, 20, 3].to_vec()
            ),
            -1
        );
    }
}
