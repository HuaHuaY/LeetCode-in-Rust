pub struct Solution;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let len = piles.len();
        let mut pre_sum = vec![0; piles.len() + 1];
        for (i, n) in piles.iter().enumerate() {
            pre_sum[i + 1] = pre_sum[i] + *n;
        }
        let mut dp = vec![vec![i32::MIN; len + 1]; len + 1];
        dp[len] = vec![0; len + 1];
        for i in (0..len).rev() {
            for m in 1..=len - i {
                for x in 1..=2 * m {
                    if i + x > len {
                        break;
                    }
                    dp[i][m] = dp[i][m]
                        .max(pre_sum[i + x] - pre_sum[i] - dp[i + x][x.max(m).min(len - i - x)]);
                }
            }
        }
        ((piles.into_iter().sum::<i32>() - dp[0][1]) >> 1) + dp[0][1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::stone_game_ii([2, 7, 9, 4, 4].to_vec()), 10);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::stone_game_ii([1, 2, 3, 4, 5, 100].to_vec()), 104);
    }
}
