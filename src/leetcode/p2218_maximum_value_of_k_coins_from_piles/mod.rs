pub struct Solution;

impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![-1; k + 1];
        dp[0] = 0;
        for pile in piles {
            for i in (1..=k).rev() {
                pile.iter().take(i).enumerate().fold(0, |acc, (idx, &j)| {
                    dp[i] = dp[i].max(dp[i - idx - 1] + acc + j);
                    acc + j
                });
            }
        }
        dp[k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_value_of_coins(vec_vec![[1, 100, 3], [7, 8, 9]], 2),
            101
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_value_of_coins(
                vec_vec![
                    [100],
                    [100],
                    [100],
                    [100],
                    [100],
                    [100],
                    [1, 1, 1, 1, 1, 1, 700]
                ],
                7
            ),
            706
        );
    }
}
