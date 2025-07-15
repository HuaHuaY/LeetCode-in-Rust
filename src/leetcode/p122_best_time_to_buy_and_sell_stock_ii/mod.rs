pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_without_stock = 0;
        let mut max_with_stock = -prices[0];
        for price in prices.into_iter().skip(1) {
            let pre_without_stock = max_without_stock;
            max_without_stock = pre_without_stock.max(max_with_stock + price);
            max_with_stock = max_with_stock.max(pre_without_stock - price);
        }
        max_without_stock
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_profit([7, 1, 5, 3, 6, 4].to_vec()), 7);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_profit([1, 2, 3, 4, 5].to_vec()), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::max_profit([7, 6, 4, 3, 1].to_vec()), 0);
    }
}
