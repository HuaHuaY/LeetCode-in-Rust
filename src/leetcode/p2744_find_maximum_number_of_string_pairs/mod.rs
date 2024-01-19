pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let mut set = HashSet::new();
        let mut result = 0;
        for word in words {
            let mut bytes = word.into_bytes();
            match set.get(&bytes) {
                Some(_) => result += 1,
                None => {
                    bytes.reverse();
                    set.insert(bytes);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecString;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::maximum_number_of_string_pairs(
                ["cd", "ac", "dc", "ca", "zz"].to_vec_string()
            ),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::maximum_number_of_string_pairs(["ab", "ba", "cc"].to_vec_string()),
            1
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::maximum_number_of_string_pairs(["aa", "ab"].to_vec_string()),
            0
        );
    }
}
