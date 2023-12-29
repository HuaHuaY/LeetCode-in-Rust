pub struct Solution;

impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let mut min = [i32::MAX; 2];
        for price in prices {
            if price < min[1] {
                if price >= min[0] {
                    min[1] = price;
                } else {
                    min[1] = min[0];
                    min[0] = price;
                }
            }
        }
        let sub = money - min[0] - min[1];
        if sub >= 0 {
            sub
        } else {
            money
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::buy_choco([1, 2, 2].to_vec(), 3), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::buy_choco([3, 2, 3].to_vec(), 3), 3);
    }
}
