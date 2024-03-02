pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        fn dfs(graph: &[Vec<i32>], pre: i32, idx: usize) -> i32 {
            let mut res = 1;
            for &next in &graph[idx] {
                if next != pre {
                    res += dfs(graph, idx as i32, next as usize);
                }
            }
            res
        }

        let set = restricted.into_iter().collect::<HashSet<_>>();

        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for edge in edges {
            if !set.contains(&edge[0]) && !set.contains(&edge[1]) {
                graph[edge[0] as usize].push(edge[1]);
                graph[edge[1] as usize].push(edge[0]);
            }
        }

        dfs(&graph, -1, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::reachable_nodes(
                7,
                [[0, 1], [1, 2], [3, 1], [4, 0], [0, 5], [5, 6]].to_vec_vec(),
                [4, 5].to_vec()
            ),
            4
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::reachable_nodes(
                7,
                [[0, 1], [0, 2], [0, 5], [0, 4], [3, 2], [6, 5]].to_vec_vec(),
                [4, 2, 1].to_vec()
            ),
            3
        )
    }
}
