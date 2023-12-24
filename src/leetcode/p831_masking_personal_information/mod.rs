pub struct Solution;

impl Solution {
    pub fn mask_pii(s: String) -> String {
        if s.chars().any(|c| c == '@') {
            let s = s
                .chars()
                .map(|c| {
                    if c.is_ascii_uppercase() {
                        (c as u8 - b'A' + b'a') as char
                    } else {
                        c
                    }
                })
                .collect::<String>();
            let split = s.split('@').collect::<Vec<_>>();
            let name = split[0].as_bytes();
            format!(
                "{}*****{}@{}",
                name[0] as char,
                name[name.len() - 1] as char,
                split[1]
            )
        } else {
            let s = s.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
            let len = s.len();
            let pre = match len {
                10 => "",
                11 => "+*-",
                12 => "+**-",
                13 => "+***-",
                _ => unreachable!(),
            };
            format!("{}***-***-{}", pre, &s[len - 4..len])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::mask_pii("LeetCode@LeetCode.com".to_string()),
            "l*****e@leetcode.com"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::mask_pii("AB@qq.com".to_string()),
            "a*****b@qq.com"
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::mask_pii("1(234)567-890".to_string()),
            "***-***-7890"
        );
    }
}
