pub struct Solution;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut dp = Vec::with_capacity(days.len());
        dp.push(*costs.iter().min().unwrap());
        for i in 1..days.len() {
            dp.push(dp[i - 1] + costs[0]);
            if days[i] - 7 > 0 {
                match days[0..i].binary_search(&(days[i] - 7)) {
                    Ok(j) => dp[i] = dp[i].min(costs[1] + dp[j]),
                    Err(j) => dp[i] = dp[i].min(costs[1] + if j > 0 { dp[j - 1] } else { 0 }),
                }
                if days[i] - 30 > 0 {
                    match days[0..i].binary_search(&(days[i] - 30)) {
                        Ok(j) => dp[i] = dp[i].min(costs[2] + dp[j]),
                        Err(j) => dp[i] = dp[i].min(costs[2] + if j > 0 { dp[j - 1] } else { 0 }),
                    }
                } else {
                    dp[i] = dp[i].min(costs[2]);
                }
            } else {
                dp[i] = dp[i].min(costs[1]).min(costs[2]);
            }
        }
        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::mincost_tickets([1, 4, 6, 7, 8, 20].to_vec(), [2, 7, 15].to_vec()),
            11
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::mincost_tickets(
                [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31].to_vec(),
                [2, 7, 15].to_vec()
            ),
            17
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::mincost_tickets([1, 4, 6, 7, 8, 20].to_vec(), [7, 2, 15].to_vec()),
            6
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::mincost_tickets(
                [1, 2, 3, 4, 6, 8, 9, 10, 13, 14, 16, 17, 19, 21, 24, 26, 27, 28, 29].to_vec(),
                [3, 14, 50].to_vec()
            ),
            50
        );
    }
}
