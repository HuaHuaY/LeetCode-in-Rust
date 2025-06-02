pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut pre_rate = ratings[0];
        let mut inc = 1;
        let mut des = 0;
        let mut result = 1;
        for rate in ratings.into_iter().skip(1) {
            match rate.cmp(&pre_rate) {
                Ordering::Greater => {
                    if des > 0 {
                        inc = 1;
                        des = 0;
                    }
                    inc += 1;
                    result += inc;
                }
                Ordering::Equal => {
                    if des > 0 {
                        des = 0;
                    }
                    inc = 1;
                    result += inc;
                }
                Ordering::Less => {
                    des += 1;
                    if des == inc {
                        des += 1;
                    }
                    result += des;
                }
            }
            pre_rate = rate;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::candy([1, 0, 2].to_vec()), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::candy([1, 2, 2].to_vec()), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::candy([1, 6, 10, 8, 7, 3, 2].to_vec()), 18);
    }
}
