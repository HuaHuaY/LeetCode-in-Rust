pub struct Solution;

use std::ops::RemAssign;

impl Solution {
    pub fn min_non_zero_product(p: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        fn pow(mut x: i64, p: i32) -> i64 {
            x.rem_assign(MOD);
            let mut res = 1;
            for _ in 0..p {
                res = (res * x).rem_euclid(MOD);
                x = (x * x).rem_euclid(MOD);
            }
            res
        }

        let k: i64 = (1 << p) - 1;
        (k.rem_euclid(MOD) * pow(k - 1, p - 1)).rem_euclid(MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_non_zero_product(1), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_non_zero_product(2), 6);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::min_non_zero_product(3), 1512);
    }
}
