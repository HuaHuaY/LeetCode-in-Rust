pub struct Solution;

impl Solution {
    pub fn find_integers(n: i32) -> i32 {
        fn dp(
            cache: &mut [[i32; 2]],
            bytes: &[u8],
            i: usize,
            last1: bool,
            limit: bool,
            number: bool,
        ) -> i32 {
            if i == bytes.len() {
                return 1;
            }
            if !limit && cache[i][last1 as usize] != -1 {
                return cache[i][last1 as usize];
            }
            let mut result = 0;
            if !number {
                result += dp(cache, bytes, i + 1, false, false, false);
            } else {
                result += dp(cache, bytes, i + 1, false, limit && bytes[i] == b'0', true);
            }
            if !last1 && (!limit || bytes[i] == b'1') {
                result += dp(cache, bytes, i + 1, true, limit, true);
            }
            if !limit {
                cache[i][last1 as usize] = result;
            }
            result
        }

        let bytes = format!("{:b}", n).into_bytes();
        dp(
            &mut vec![[-1; 2]; bytes.len()],
            &bytes,
            0,
            false,
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
        assert_eq!(Solution::find_integers(5), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_integers(1), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::find_integers(2), 3);
    }
}
