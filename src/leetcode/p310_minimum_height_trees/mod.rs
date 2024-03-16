pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }
        let mut n = n as usize;
        let edges = edges.into_iter().fold(vec![vec![]; n], |mut edges, v| {
            edges[v[0] as usize].push(v[1] as usize);
            edges[v[1] as usize].push(v[0] as usize);
            edges
        });
        let mut degree = edges.iter().map(|v| v.len()).collect::<Vec<_>>();
        let mut queue = degree
            .iter()
            .enumerate()
            .filter_map(|(i, &d)| (d == 1).then_some(i))
            .collect::<VecDeque<_>>();
        while n > 2 {
            let len = queue.len();
            n -= len;
            for _ in 0..len {
                let i = queue.pop_front().unwrap();
                for &j in &edges[i] {
                    degree[j] -= 1;
                    if degree[j] == 1 {
                        queue.push_back(j);
                    }
                }
            }
        }
        queue.into_iter().map(|i| i as i32).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_min_height_trees(4, [[1, 0], [1, 2], [1, 3]].to_vec_vec()),
            [1]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_min_height_trees(
                6,
                [[3, 0], [3, 1], [3, 2], [3, 4], [5, 4]].to_vec_vec()
            ),
            [3, 4]
        );
    }
}
