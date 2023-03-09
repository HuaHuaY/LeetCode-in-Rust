pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        fn check_count(s1: &str, s2: &str) -> bool {
            let count = |str: &str| {
                str.bytes().fold(vec![0; 26], |mut vec, byte| {
                    vec[(byte - b'a') as usize] += 1;
                    vec
                })
            };
            count(s1) == count(s2)
        }

        fn dfs(
            map: &mut HashMap<(usize, usize, usize), bool>,
            s1: &str,
            s2: &str,
            i: usize,
            j: usize,
            length: usize,
        ) -> bool {
            if let Some(bool) = map.get(&(i, j, length)) {
                return *bool;
            }
            if s1[i..i + length] == s2[j..j + length] {
                map.insert((i, j, length), true);
                return true;
            }
            if !check_count(&s1[i..i + length], &s2[j..j + length]) {
                map.insert((i, j, length), false);
                return false;
            }

            let result = (1..length).any(|k| {
                let unchange_left = dfs(map, s1, s2, i, j, k);
                let unchange_right = dfs(map, s1, s2, i + k, j + k, length - k);
                if unchange_left && unchange_right {
                    return true;
                }

                let change_left = dfs(map, s1, s2, i, j + length - k, k);
                let change_right = dfs(map, s1, s2, i + k, j, length - k);
                change_left && change_right
            });
            map.insert((i, j, length), result);
            result
        }

        dfs(&mut HashMap::new(), &s1, &s2, 0, 0, s1.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_scramble(
            "great".to_string(),
            "rgeat".to_string()
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_scramble(
            "abcde".to_string(),
            "caebd".to_string()
        ));
    }

    #[test]
    fn test3() {
        assert!(Solution::is_scramble("a".to_string(), "a".to_string()));
    }
}
