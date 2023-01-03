pub struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut dp = vec![0; n as usize];
        dp[0] = 1;
        let mut p2 = 0;
        let mut p3 = 0;
        let mut p5 = 0;
        for i in 1..n as usize {
            dp[i] = (2 * dp[p2]).min(3 * dp[p3]).min(5 * dp[p5]);
            if dp[i] == 2 * dp[p2] {
                p2 += 1;
            }
            if dp[i] == 3 * dp[p3] {
                p3 += 1;
            }
            if dp[i] == 5 * dp[p5] {
                p5 += 1;
            }
        }
        dp[n as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::nth_ugly_number(10), 12);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::nth_ugly_number(1), 1);
    }
}
