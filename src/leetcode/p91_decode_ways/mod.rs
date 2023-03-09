pub struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut count_pre_pre = 0;
        let mut count_pre = 1;
        let mut byte_pre = b'0';
        for b in s.bytes() {
            match byte_pre {
                b'0' => {
                    if b == b'0' {
                        return 0;
                    }
                    count_pre_pre = count_pre;
                }
                b'1' => {
                    if b == b'0' {
                        count_pre = count_pre_pre;
                    } else {
                        count_pre += count_pre_pre;
                        count_pre_pre = count_pre - count_pre_pre;
                    }
                }
                b'2' => match b {
                    b'0' => count_pre = count_pre_pre,
                    b'1'..=b'6' => {
                        count_pre += count_pre_pre;
                        count_pre_pre = count_pre - count_pre_pre;
                    }
                    _ => count_pre_pre = count_pre,
                },
                _ => {
                    if b == b'0' {
                        return 0;
                    }
                    count_pre_pre = count_pre;
                }
            }
            byte_pre = b;
        }
        count_pre
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::num_decodings("12".to_string()), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::num_decodings("226".to_string()), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::num_decodings("06".to_string()), 0);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::num_decodings("10".to_string()), 1);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::num_decodings("1201234".to_string()), 3);
    }
}
