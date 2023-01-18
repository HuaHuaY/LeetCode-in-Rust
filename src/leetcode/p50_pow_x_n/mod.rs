pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        Solution::foo1(x, n)
    }

    fn foo1(mut x: f64, n: i32) -> f64 {
        let mut result = 1.0f64;
        let mut n_abs = (n as i64).abs();
        while n_abs > 0 {
            if n_abs & 1 == 1 {
                result *= x;
            }
            x *= x;
            n_abs >>= 1;
        }
        if n >= 0 {
            result
        } else {
            1.0 / result
        }
    }

    fn foo2(x: f64, n: i32) -> f64 {
        match n {
            0 => return 1.0,
            1 => return x,
            -1 => return 1.0 / x,
            _ => {
                let y = Solution::my_pow(x, n >> 1);
                y * y * Solution::my_pow(x, n & 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::my_pow(2.00000, 10), 2.00000f64.powi(10));
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::my_pow(2.10000, 3), 2.10000f64.powi(3));
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::my_pow(2.00000, -2), 2.00000f64.powi(-2));
    }
}
