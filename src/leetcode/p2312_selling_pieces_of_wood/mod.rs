pub struct Solution;

impl Solution {
    pub fn selling_wood(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64 {
        let m = m as usize;
        let n = n as usize;
        let mut prices = prices
            .into_iter()
            .fold(vec![vec![0; n + 1]; m + 1], |mut map, v| {
                map[v[0] as usize][v[1] as usize] = v[2] as i64;
                map
            });
        for i in 1..=m {
            for j in 1..=n {
                let mut max = prices[i][j];
                for k in 1..=i / 2 {
                    max = max.max(prices[k][j] + prices[i - k][j]);
                }
                for k in 1..=j / 2 {
                    max = max.max(prices[i][k] + prices[i][j - k]);
                }
                prices[i][j] = max;
            }
        }
        prices[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::selling_wood(3, 5, [[1, 4, 2], [2, 2, 7], [2, 1, 3]].to_vec_vec()),
            19
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::selling_wood(4, 6, [[3, 2, 10], [1, 4, 2], [4, 1, 3]].to_vec_vec()),
            32
        );
    }
}
