pub struct Solution;

impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        let len = piles.len();
        piles.sort_unstable();
        piles[len / 3..].iter().step_by(2).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_coins([2, 4, 1, 2, 7, 8].to_vec()), 9);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_coins([2, 4, 5].to_vec()), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::max_coins([9, 8, 7, 6, 5, 1, 2, 3, 4].to_vec()),
            18
        );
    }
}
