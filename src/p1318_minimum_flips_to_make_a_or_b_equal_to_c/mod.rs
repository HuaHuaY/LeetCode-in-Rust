pub struct Solution {}

impl Solution {
    pub fn min_flips(mut a: i32, mut b: i32, mut c: i32) -> i32 {
        let mut a_remainder;
        let mut b_remainder;
        let mut c_remainder;
        let mut answer = 0;

        while a != 0 || b != 0 || c != 0 {
            a_remainder = a % 2;
            b_remainder = b % 2;
            c_remainder = c % 2;
            if c_remainder == 0 {
                answer += a_remainder + b_remainder;
            } else {
                answer += if a_remainder + b_remainder == 0 { 1 } else { 0 };
            }
            a >>= 1;
            b >>= 1;
            c >>= 1;
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::min_flips(2, 6, 5), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_flips(4, 2, 7), 1);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::min_flips(1, 2, 3), 0);
    }
}
