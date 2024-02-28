pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn root_count(edges: Vec<Vec<i32>>, guesses: Vec<Vec<i32>>, k: i32) -> i32 {
        fn count(
            graph: &[Vec<usize>],
            guesses: &HashSet<(usize, usize)>,
            pre: i32,
            idx: usize,
        ) -> i32 {
            let mut result = 0;
            for &next in &graph[idx] {
                if next as i32 != pre {
                    if guesses.contains(&(idx, next)) {
                        result += 1;
                    }
                    result += count(graph, guesses, idx as i32, next);
                }
            }
            result
        }

        fn reroot(
            graph: &[Vec<usize>],
            guesses: &HashSet<(usize, usize)>,
            pre: i32,
            idx: usize,
            k: i32,
            val: i32,
        ) -> i32 {
            let mut result = if val >= k { 1 } else { 0 };
            for &next in &graph[idx] {
                if next as i32 != pre {
                    let mut next_val = val;
                    if guesses.contains(&(idx, next)) {
                        next_val -= 1;
                    }
                    if guesses.contains(&(next, idx)) {
                        next_val += 1;
                    }
                    result += reroot(graph, guesses, idx as i32, next, k, next_val);
                }
            }
            result
        }

        let guesses = guesses
            .into_iter()
            .map(|x| (x[0] as usize, x[1] as usize))
            .collect::<HashSet<_>>();
        let mut graph = vec![vec![]; edges.len() + 1];
        for edge in edges {
            graph[edge[0] as usize].push(edge[1] as usize);
            graph[edge[1] as usize].push(edge[0] as usize);
        }

        let init = count(&graph, &guesses, -1, 0);
        reroot(&graph, &guesses, -1, 0, k, init)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::root_count(
                [[0, 1], [1, 2], [1, 3], [4, 2]].to_vec_vec(),
                [[1, 3], [0, 1], [1, 0], [2, 4]].to_vec_vec(),
                3
            ),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::root_count(
                [[0, 1], [1, 2], [2, 3], [3, 4]].to_vec_vec(),
                [[1, 0], [3, 4], [2, 1], [3, 2]].to_vec_vec(),
                1
            ),
            5
        );
    }
}
