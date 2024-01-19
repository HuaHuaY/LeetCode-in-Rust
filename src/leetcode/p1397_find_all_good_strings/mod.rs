pub struct Solution;

impl Solution {
    pub fn find_good_strings(n: i32, s1: String, s2: String, evil: String) -> i32 {
        struct Config<'a> {
            n: usize,
            s1: &'a [u8],
            s2: &'a [u8],
            evil: &'a [u8],
            evil_nextval: Vec<i32>,
        }

        fn get_kmp_nextval(evil: &[u8]) -> Vec<i32> {
            let mut nextval = vec![-1; evil.len()];
            let mut i = 0;
            let mut j = -1;
            while i < evil.len() - 1 {
                if j == -1 || evil[i] == evil[j as usize] {
                    i += 1;
                    j += 1;
                    if evil[i] != evil[j as usize] {
                        nextval[i] = j;
                    } else {
                        nextval[i] = nextval[j as usize];
                    }
                } else {
                    j = nextval[j as usize];
                }
            }
            nextval
        }

        fn dp(
            cache: &mut [Vec<i32>],
            config: &Config,
            i: usize,
            str: &mut [u8],
            p: usize,
            high_limit: bool,
            low_limit: bool,
        ) -> i32 {
            if p == config.evil.len() || str[..i].as_ref() > &config.s2[..i] {
                return 0;
            }
            if i == config.n {
                return if str as &[u8] >= config.s1 { 1 } else { 0 };
            }
            if !high_limit && !low_limit && cache[i][p] != -1 {
                return cache[i][p];
            }

            let low = if low_limit { config.s1[i] } else { b'a' };
            let high = if high_limit { config.s2[i] } else { b'z' };

            let mut result = 0;
            for d in low..=high {
                str[i] = d;
                result += dp(
                    cache,
                    config,
                    i + 1,
                    str,
                    if d == config.evil[p] {
                        p + 1
                    } else {
                        let mut t = config.evil_nextval[p];
                        while t >= 0 && d != config.evil[t as usize] {
                            t = config.evil_nextval[t as usize];
                        }
                        if t == -1 {
                            0
                        } else {
                            t as usize + 1
                        }
                    },
                    high_limit && d == high,
                    low_limit && d == low,
                );
                result %= 1_000_000_007;
            }

            if !high_limit && !low_limit {
                cache[i][p] = result;
            }

            result
        }

        if s1.starts_with(&evil) && s2.starts_with(&evil) {
            return 0;
        }
        let n = n as usize;
        dp(
            &mut vec![vec![-1; evil.len()]; n],
            &Config {
                n,
                s1: s1.as_bytes(),
                s2: s2.as_bytes(),
                evil: evil.as_bytes(),
                evil_nextval: get_kmp_nextval(evil.as_bytes()),
            },
            0,
            &mut vec![b'0'; n],
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
        assert_eq!(
            Solution::find_good_strings(2, "aa".to_string(), "da".to_string(), "b".to_string()),
            51
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_good_strings(
                8,
                "leetcode".to_string(),
                "leetgoes".to_string(),
                "leet".to_string()
            ),
            0
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::find_good_strings(2, "gx".to_string(), "gz".to_string(), "x".to_string()),
            2
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::find_good_strings(
                8,
                "pzdanyao".to_string(),
                "wgpmtywi".to_string(),
                "sdka".to_string()
            ),
            500543753
        );
    }
}
