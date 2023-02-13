pub struct Solution;

impl Solution {
    pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        let calculate = |n: i32| (n % 1_000_000_007 + 1_000_000_007) % 1_000_000_007;
        let mut dp = vec![vec![0; 6]; n as usize];
        let mut result = vec![0; n as usize];
        dp[0] = vec![1; 6];
        result[0] = 6;
        for i in 1..n as usize {
            for (j, roll_max) in roll_max.iter().enumerate() {
                dp[i][j] = result[i - 1];
                let idx = i as i32 - roll_max - 1;
                match idx {
                    0.. => dp[i][j] -= calculate(result[idx as usize] - dp[idx as usize][j]),
                    -1 => dp[i][j] -= 1,
                    _ => (),
                }
                dp[i][j] = calculate(dp[i][j]);
                result[i] = calculate(result[i] + dp[i][j]);
            }
        }
        result[n as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::die_simulator(2, [1, 1, 2, 2, 2, 3].to_vec()), 34);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::die_simulator(2, [1, 1, 1, 1, 1, 1].to_vec()), 30);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::die_simulator(3, [1, 1, 1, 2, 2, 3].to_vec()), 181);
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::die_simulator(20, [8, 5, 10, 8, 7, 2].to_vec()),
            822005673
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::die_simulator(30, [2, 3, 1, 2, 1, 2].to_vec()),
            753152086
        );
    }
}
