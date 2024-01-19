pub struct Solution;

impl Solution {
    pub fn count_special_numbers(n: i32) -> i32 {
        fn dp(
            cache: &mut [[i32; 1 << 10]],
            bytes: &[u8],
            i: usize,
            mask: usize,
            limit: bool,
            number: bool,
        ) -> i32 {
            if i == bytes.len() {
                return if number { 1 } else { 0 };
            }
            if !limit && cache[i][mask] != -1 {
                return cache[i][mask];
            }
            let mut result = 0;
            if !number {
                result += dp(cache, bytes, i + 1, mask, false, false);
            }
            let low = 1 - number as usize;
            let high = if limit { (bytes[i] - b'0') as usize } else { 9 };
            for d in low..=high {
                if (mask >> d) & 1 == 0 {
                    result += dp(
                        cache,
                        bytes,
                        i + 1,
                        mask | (1 << d),
                        limit && d == high,
                        true,
                    );
                }
            }
            if !limit {
                cache[i][mask] = result;
            }
            result
        }

        let bytes = n.to_string().into_bytes();
        dp(
            &mut vec![[-1; 1 << 10]; bytes.len()],
            &bytes,
            0,
            0,
            true,
            false,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_special_numbers(20), 19);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count_special_numbers(5), 5);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::count_special_numbers(135), 110);
    }
}
