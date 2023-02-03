pub struct Solution;

impl Solution {
    pub fn get_maximum_consecutive(mut coins: Vec<i32>) -> i32 {
        coins.sort_unstable();
        let mut max = 0;
        for coin in coins {
            if coin > max + 1 {
                break;
            }
            max += coin;
        }
        max + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::get_maximum_consecutive([1, 3].to_vec()), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::get_maximum_consecutive([1, 1, 1, 4].to_vec()), 8);
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::get_maximum_consecutive([1, 4, 10, 3, 1].to_vec()),
            20
        );
    }
}
