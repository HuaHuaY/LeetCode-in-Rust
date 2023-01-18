pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut answer: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
        for str in strs {
            let mut str_v = str.bytes().collect::<Vec<_>>();
            str_v.sort_unstable();
            answer.entry(str_v).or_default().push(str);
        }
        answer.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::{ToSortVecVec, ToVecString},
        vec_vec,
    };

    #[test]
    fn test1() {
        assert_eq!(
            Solution::group_anagrams(["eat", "tea", "tan", "ate", "nat", "bat"].to_vec_string())
                .to_sort_vec_vec(),
            vec_vec![["bat"], ["nat", "tan"], ["ate", "eat", "tea"]].to_sort_vec_vec()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::group_anagrams([""].to_vec_string()).to_sort_vec_vec(),
            vec_vec![[""]].to_sort_vec_vec()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::group_anagrams(["a"].to_vec_string()).to_sort_vec_vec(),
            vec_vec![["a"]].to_sort_vec_vec()
        );
    }
}
