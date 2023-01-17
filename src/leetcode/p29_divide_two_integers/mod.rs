pub struct Solution;

impl Solution {
    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }

        let sign = ((dividend as u32 >> 31) ^ (divisor as u32 >> 31)) as i32;
        if dividend > 0 {
            dividend = -dividend;
        }
        if divisor > 0 {
            divisor = -divisor;
        }

        let mut vec = vec![divisor];
        while divisor > dividend - divisor {
            divisor <<= 1;
            vec.push(divisor);
        }

        let mut result = 0;
        for (idx, num) in vec.into_iter().enumerate().rev() {
            while dividend <= num {
                dividend -= num;
                result -= 1 << idx as i32;
            }
        }

        if sign == 1 {
            result
        } else if result == i32::MIN {
            i32::MAX
        } else {
            -result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::divide(10, 3), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::divide(7, -3), -2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::divide(-2147483648, 1), -2147483648);
    }
}
