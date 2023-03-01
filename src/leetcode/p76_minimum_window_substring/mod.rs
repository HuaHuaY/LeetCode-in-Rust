pub struct Solution;

use std::collections::{hash_map::Entry, HashMap};

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s = s.as_bytes();
        let mut t = t.bytes().fold(HashMap::new(), |mut map, byte| {
            map.entry(byte).and_modify(|e| *e -= 1).or_insert(-1);
            map
        });
        let mut result = (0, usize::MAX);
        let mut left = 0;
        for (right, byte) in s.iter().enumerate() {
            if let Entry::Occupied(mut entry) = t.entry(*byte) {
                *entry.get_mut() += 1;
                while t.iter().all(|(_, v)| *v >= 0) {
                    while let Entry::Vacant(_) = t.entry(s[left]) {
                        left += 1;
                    }
                    if result.1 - result.0 > right - left {
                        result = (left, right);
                    }
                    t.entry(s[left]).and_modify(|e| *e -= 1);
                    left += 1;
                }
            }
        }
        if result.1 == usize::MAX {
            "".to_string()
        } else {
            s[result.0..=result.1]
                .iter()
                .map(|byte| *byte as char)
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_window("a".to_string(), "a".to_string()), "a");
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::min_window("a".to_string(), "aa".to_string()), "");
    }
}
