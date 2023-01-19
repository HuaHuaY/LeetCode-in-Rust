pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for log in logs {
            map.entry(log[0])
                .or_insert_with(HashSet::new)
                .insert(log[1]);
        }
        let map = map
            .into_iter()
            .map(|(k, v)| (k, v.len()))
            .collect::<HashMap<_, _>>();
        let mut res = vec![0; k as usize];
        for v in map.into_values() {
            res[v - 1] += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::finding_users_active_minutes(
                [[0, 5], [1, 2], [0, 2], [0, 5], [1, 3]].to_vec_vec(),
                5
            ),
            [0, 2, 0, 0, 0]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::finding_users_active_minutes([[1, 1], [2, 2], [2, 3]].to_vec_vec(), 4),
            [1, 1, 0, 0]
        );
    }
}
