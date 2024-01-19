pub struct Solution;

impl Solution {
    pub fn count(num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32 {
        struct Config<'a> {
            num1: &'a [u8],
            num2: &'a [u8],
            min_sum: i32,
            max_sum: i32,
        }

        fn dp(
            cache: &mut [Vec<i32>],
            config: &Config,
            i: usize,
            sum: i32,
            high_limit: bool,
            low_limit: bool,
        ) -> i32 {
            if sum > config.max_sum {
                return 0;
            }
            if i == config.num2.len() {
                return if sum >= config.min_sum { 1 } else { 0 };
            }
            if !high_limit && !low_limit && cache[i][sum as usize] != -1 {
                return cache[i][sum as usize];
            }

            let low = if low_limit { config.num1[i] - b'0' } else { 0 };
            let high = if high_limit { config.num2[i] - b'0' } else { 9 };

            let mut result = 0;
            for d in low..=high {
                result += dp(
                    cache,
                    config,
                    i + 1,
                    sum + d as i32,
                    high_limit && d == high,
                    low_limit && d == low,
                );
                result %= 1_000_000_007;
            }

            if !high_limit && !low_limit {
                cache[i][sum as usize] = result;
            }

            result
        }

        let mut bytes1 = vec![b'0'; num2.len() - num1.len()];
        bytes1.extend(num1.into_bytes());
        let bytes2 = num2.into_bytes();
        dp(
            &mut vec![vec![-1; (9 * bytes2.len()).min(max_sum as usize) + 1]; bytes2.len()],
            &Config {
                num1: &bytes1,
                num2: &bytes2,
                min_sum,
                max_sum,
            },
            0,
            0,
            true,
            true,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count("1".to_string(), "12".to_string(), 1, 8), 11);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count("1".to_string(), "5".to_string(), 1, 5), 5);
    }
}
