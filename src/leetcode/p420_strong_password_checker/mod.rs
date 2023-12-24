pub struct Solution;

impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let chars = password.chars().collect::<Vec<_>>();
        let mut has_lower = false;
        let mut has_upper = false;
        let mut has_digit = false;
        for c in &chars {
            if c.is_lowercase() {
                has_lower = true;
            } else if c.is_uppercase() {
                has_upper = true;
            } else if c.is_ascii_digit() {
                has_digit = true;
            }
        }
        let kind = vec![has_lower, has_upper, has_digit]
            .into_iter()
            .filter(|e| *e)
            .count() as i32;

        let len = password.len();
        match len {
            0..=5 => (6 - len as i32).max(3 - kind),
            6..=20 => {
                let mut result = 0;
                let mut count = 1;
                for i in 1..chars.len() {
                    if chars[i] == chars[i - 1] {
                        count += 1;
                        if count == 3 {
                            count = 0;
                            result += 1;
                        }
                    } else {
                        count = 1;
                    }
                }
                result
            }
            21..=50 => {
                let delete = len - 20;
                let mut replace = 0;
                let mut mod_3 = vec![0; 2];
                let mut i = 0;
                while i < len {
                    let mut j = i;
                    while j + 1 < len && chars[j] == chars[j + 1] {
                        j += 1;
                    }
                    if j - i + 1 >= 3 {
                        replace += (j - i + 1) / 3;
                        let tmp = (j - i + 1) % 3;
                        if tmp != 2 {
                            mod_3[tmp] += 1;
                        }
                    }
                    i = j + 1;
                }
                let mut tmp = delete;
                for (i, n) in mod_3.into_iter().enumerate() {
                    if tmp >= n * (i + 1) {
                        tmp -= n * (i + 1);
                        replace -= n;
                    } else {
                        replace -= tmp / (i + 1);
                        tmp = 0;
                        break;
                    }
                }
                replace -= tmp / 3;
                delete as i32 + (replace as i32).max(3 - kind)
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::strong_password_checker("a".to_string()), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::strong_password_checker("aA1".to_string()), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::strong_password_checker("1337C0d3".to_string()), 0);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::strong_password_checker("aaa123".to_string()), 1);
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::strong_password_checker("FFFFFFFFFFFFFFF11111111111111111111AAA".to_string()),
            23
        );
    }
}
