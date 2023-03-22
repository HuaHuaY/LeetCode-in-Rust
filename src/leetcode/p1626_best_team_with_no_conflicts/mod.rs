pub struct Solution;

impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let mut vec = scores.into_iter().zip(ages).collect::<Vec<_>>();
        vec.sort_unstable();
        let mut dp = vec![0; vec.len()];
        let mut result = 0;
        for i in 0..vec.len() {
            for j in 0..i {
                if vec[i].1 >= vec[j].1 {
                    dp[i] = dp[i].max(dp[j]);
                }
            }
            dp[i] += vec[i].0;
            result = result.max(dp[i]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::best_team_score([1, 3, 5, 10, 15].to_vec(), [1, 2, 3, 4, 5].to_vec()),
            34
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::best_team_score([4, 5, 6, 5].to_vec(), [2, 1, 2, 1].to_vec()),
            16
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::best_team_score([1, 2, 3, 5].to_vec(), [8, 9, 10, 1].to_vec()),
            6
        );
    }
}
