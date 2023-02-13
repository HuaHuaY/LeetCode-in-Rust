pub struct Solution;

impl Solution {
    pub fn fill_cups(mut amount: Vec<i32>) -> i32 {
        amount.sort_unstable();
        if amount[0] <= amount[2] - amount[1] {
            amount[2]
        } else {
            (amount.into_iter().sum::<i32>() + 1) / 2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::fill_cups([1, 4, 2].to_vec()), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::fill_cups([5, 4, 4].to_vec()), 7);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::fill_cups([5, 0, 0].to_vec()), 5);
    }
}
