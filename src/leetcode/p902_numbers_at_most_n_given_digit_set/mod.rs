pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, mut n: i32) -> i32 {
        let digits = digits
            .into_iter()
            .map(|i| i.into_bytes()[0] - b'0')
            .collect::<Vec<u8>>();

        let len = digits.len() as i32;
        let mut last_a = 1;
        let mut result = 1;

        let mut n_len = 0;
        let mut b;
        while n != 0 {
            b = (n % 10) as u8;
            n /= 10;
            n_len += 1;

            let mut t = 0;
            for d in &digits {
                t += match d.cmp(&b) {
                    Ordering::Greater => break,
                    Ordering::Equal => result,
                    Ordering::Less => last_a,
                };
            }
            result = t;
            last_a *= len;
        }

        result
            + if len == 1 {
                n_len as i32 - 1
            } else {
                (len.pow(n_len) - len) / (len - 1)
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecString;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::at_most_n_given_digit_set(["1", "3", "5", "7"].to_vec_string(), 100),
            20
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::at_most_n_given_digit_set(["1", "4", "9"].to_vec_string(), 1000000000),
            29523
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::at_most_n_given_digit_set(["7"].to_vec_string(), 8),
            1
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::at_most_n_given_digit_set(["3", "4", "8"].to_vec_string(), 4),
            2
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::at_most_n_given_digit_set(["9"].to_vec_string(), 55),
            1
        );
    }
}
