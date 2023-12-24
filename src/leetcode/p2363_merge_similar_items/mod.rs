pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len = items1.len() + items2.len();
        let mut vec = items1
            .into_iter()
            .chain(items2)
            .fold(HashMap::with_capacity(len), |mut map, item| {
                map.entry(item[0])
                    .and_modify(|e| *e += item[1])
                    .or_insert(item[1]);
                map
            })
            .into_iter()
            .map(|(k, v)| vec![k, v])
            .collect::<Vec<_>>();
        vec.sort_unstable();
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::merge_similar_items(
                [[1, 1], [4, 5], [3, 8]].to_vec_vec(),
                [[3, 1], [1, 5]].to_vec_vec()
            ),
            [[1, 6], [3, 9], [4, 5]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::merge_similar_items(
                [[1, 1], [3, 2], [2, 3]].to_vec_vec(),
                [[2, 1], [3, 2], [1, 3]].to_vec_vec()
            ),
            [[1, 4], [2, 4], [3, 4]]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::merge_similar_items(
                [[1, 3], [2, 2]].to_vec_vec(),
                [[7, 1], [2, 2], [1, 4]].to_vec_vec()
            ),
            [[1, 7], [2, 4], [7, 1]]
        );
    }
}
