pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut result = 0;
        let mut sign = 0;
        let mut start_match = false;
        for c in s.trim_start_matches(' ').bytes() {
            match c {
                b'0'..=b'9' => {
                    start_match = true;
                    if sign == 0 {
                        sign = 1;
                    }

                    let digit = (c - b'0') as i32;
                    if sign == 1 && result > (i32::MAX - digit) / 10 {
                        return i32::MAX;
                    }
                    if sign == -1 && sign * result < (i32::MIN + digit) / 10 {
                        return i32::MIN;
                    }
                    result = result * 10 + digit;
                }
                b'-' => {
                    if sign != 0 || start_match {
                        break;
                    }
                    sign = -1;
                }
                b'+' => {
                    if sign != 0 || start_match {
                        break;
                    }
                    sign = 1;
                }
                _ => break,
            }
        }

        result * sign
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
    }
}
