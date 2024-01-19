pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        Solution::foo2(n)
    }

    fn foo1(n: i32) -> i32 {
        fn dp(cache: &mut [i32], n: i32, bytes: &[u8], i: usize, limit: bool, number: bool) -> i32 {
            if i == bytes.len() {
                return 0;
            }
            if !limit && cache[i] != -1 {
                return cache[i];
            }
            let mut result = 0;
            if !number {
                result += dp(cache, n, bytes, i + 1, false, false);
            }
            let low = 1 - number as usize;
            let high = if limit { (bytes[i] - b'0') as usize } else { 9 };
            result += match high {
                0 => 0,
                1 if limit => n % (10i32.pow((bytes.len() - i - 1) as u32)) + 1,
                _ => 10i32.pow((bytes.len() - i - 1) as u32),
            };
            result += dp(cache, n, bytes, i + 1, limit, true);
            if low < high {
                result += (high - low) as i32 * dp(cache, n, bytes, i + 1, false, true);
            }
            if !limit {
                cache[i] = result;
            }
            result
        }

        let bytes = n.to_string().into_bytes();
        dp(&mut vec![-1; bytes.len()], n, &bytes, 0, true, false)
    }

    fn foo2(n: i32) -> i32 {
        let n = n as i64;
        let mut mulk = 1i64;
        let mut result = 0;
        while n >= mulk {
            result += (n / (mulk * 10)) * mulk + (n % (mulk * 10) - mulk + 1).max(0).min(mulk);
            mulk *= 10;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_digit_one(13), 6);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count_digit_one(0), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::count_digit_one(20), 12);
    }
}
