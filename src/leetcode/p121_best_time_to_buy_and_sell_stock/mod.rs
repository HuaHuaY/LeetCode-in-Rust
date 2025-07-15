pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut min = prices[0];
        for price in prices.into_iter().skip(1) {
            result = result.max(price - min);
            min = min.min(price);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_profit([7, 1, 5, 3, 6, 4].to_vec()), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_profit([7, 6, 4, 3, 1].to_vec()), 0);
    }
}
