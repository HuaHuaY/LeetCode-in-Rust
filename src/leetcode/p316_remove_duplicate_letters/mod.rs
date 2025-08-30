pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut count_right = s.bytes().fold([0; 26], |mut acc, b: u8| {
            acc[(b - b'a') as usize] += 1;
            acc
        });
        let mut contains = HashSet::new();
        let mut stack = Vec::new();
        for b in s.bytes() {
            count_right[(b - b'a') as usize] -= 1;
            if contains.contains(&b) {
                continue;
            }
            while let Some(&last) = stack.last() {
                if last > b && count_right[(last - b'a') as usize] > 0 {
                    stack.pop();
                    contains.remove(&last);
                } else {
                    break;
                }
            }
            stack.push(b);
            contains.insert(b);
        }
        String::from_utf8(stack).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::remove_duplicate_letters("bcabc".to_string()),
            "abc"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::remove_duplicate_letters("cbacdcbc".to_string()),
            "acdb"
        );
    }
}
