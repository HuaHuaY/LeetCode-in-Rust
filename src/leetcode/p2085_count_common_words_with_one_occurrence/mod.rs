pub struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let lens = [words1.len(), words2.len()];
        let mut sets: [HashSet<String>; 2] = Default::default();
        for (i, word) in IntoIterator::into_iter([words1, words2]).enumerate() {
            sets[i] = word
                .into_iter()
                .fold(HashMap::with_capacity(lens[i]), |mut map, e| {
                    map.entry(e).and_modify(|e| *e += 1).or_insert(1);
                    map
                })
                .into_iter()
                .filter(|(_, v)| *v == 1)
                .map(|(k, _)| k)
                .collect::<HashSet<String>>();
        }
        sets[0].intersection(&sets[1]).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecString;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_words(
                ["leetcode", "is", "amazing", "as", "is"].to_vec_string(),
                ["amazing", "leetcode", "is"].to_vec_string()
            ),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::count_words(
                ["b", "bb", "bbb"].to_vec_string(),
                ["a", "aa", "aaa"].to_vec_string()
            ),
            0
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::count_words(
                ["a", "ab"].to_vec_string(),
                ["a", "a", "a", "ab"].to_vec_string()
            ),
            1
        );
    }
}
