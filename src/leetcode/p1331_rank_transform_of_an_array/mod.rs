pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn array_rank_transform(mut arr: Vec<i32>) -> Vec<i32> {
        let mut result = arr.clone();
        arr.sort_unstable();
        arr.dedup();
        let hashmap = arr
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, i + 1))
            .collect::<HashMap<i32, usize>>();
        for i in result.iter_mut() {
            *i = *hashmap.get(i).unwrap() as i32;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::array_rank_transform(vec![40, 10, 20, 30]),
            [4, 1, 2, 3]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::array_rank_transform(vec![100, 100, 100]),
            [1, 1, 1]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
            [5, 3, 4, 2, 8, 6, 7, 1, 3]
        );
    }
}
