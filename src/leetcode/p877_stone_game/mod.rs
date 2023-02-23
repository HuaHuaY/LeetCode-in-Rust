pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        Solution::foo2(piles)
    }

    fn foo1(_piles: Vec<i32>) -> bool {
        // divided into odd and even groups
        true
    }

    fn foo2(piles: Vec<i32>) -> bool {
        let mut dp = vec![vec![0; piles.len()]; piles.len()];
        for i in (0..piles.len()).rev() {
            for j in i..piles.len() {
                if i == j {
                    dp[i][j] = piles[i];
                } else {
                    dp[i][j] = (piles[i] - dp[i + 1][j]).max(piles[j] - dp[i][j - 1]);
                }
            }
        }
        dp[0][piles.len() - 1] > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::stone_game([5, 3, 4, 5].to_vec()));
    }

    #[test]
    fn test2() {
        assert!(Solution::stone_game([3, 7, 2, 3].to_vec()));
    }
}
