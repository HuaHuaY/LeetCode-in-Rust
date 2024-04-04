pub struct Solution;

use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut e = vec![vec![]; n];
        let mut nums = vec![0; n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            e[u].push(v);
            nums[v] += 1;
        }

        let mut queue = nums
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 0)
            .map(|(i, _)| i)
            .collect::<VecDeque<_>>();
        let mut result = vec![HashSet::new(); n];
        while let Some(s) = queue.pop_front() {
            for &v in &e[s] {
                let t = result[s].clone();
                result[v].extend(t);
                result[v].insert(s as i32);
                nums[v] -= 1;
                if nums[v] == 0 {
                    queue.push_back(v);
                }
            }
        }
        result
            .into_iter()
            .map(|a| {
                let mut v = a.into_iter().collect::<Vec<_>>();
                v.sort_unstable();
                v
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::common::ToVecVec;
    use crate::vec_vec;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::get_ancestors(
                8,
                [
                    [0, 3],
                    [0, 4],
                    [1, 3],
                    [2, 4],
                    [2, 7],
                    [3, 5],
                    [3, 6],
                    [3, 7],
                    [4, 6]
                ]
                .to_vec_vec()
            ),
            vec_vec![
                [],
                [],
                [],
                [0, 1],
                [0, 2],
                [0, 1, 3],
                [0, 1, 2, 3, 4],
                [0, 1, 2, 3]
            ]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::get_ancestors(
                5,
                [
                    [0, 1],
                    [0, 2],
                    [0, 3],
                    [0, 4],
                    [1, 2],
                    [1, 3],
                    [1, 4],
                    [2, 3],
                    [2, 4],
                    [3, 4]
                ]
                .to_vec_vec()
            ),
            vec_vec![[], [0], [0, 1], [0, 1, 2], [0, 1, 2, 3]]
        );
    }
}
