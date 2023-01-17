pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let needle = needle.chars().collect::<Vec<_>>();
        let length = needle.len();
        let mut next = Vec::with_capacity(length);
        next.push(-1);
        let mut k = -1;
        let mut j = 0;
        while j < length - 1 {
            if k == -1 || needle[j] == needle[k as usize] {
                k += 1;
                j += 1;
                if needle[j] != needle[k as usize] {
                    next.push(k);
                } else {
                    next.push(next[k as usize]);
                }
            } else {
                k = next[k as usize];
            }
        }

        let mut j = 0;
        for (i, c) in haystack.chars().enumerate() {
            while j >= 0 && c != needle[j as usize] {
                j = next[j as usize];
            }
            if j < 0 {
                j = 0;
                continue;
            }
            j += 1;
            if j as usize == length {
                return (i - length + 1) as i32;
            }
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::str_str("sadbutsad".to_string(), "sad".to_string()),
            0
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::str_str("leetcode".to_string(), "leeto".to_string()),
            -1
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
    }
}
