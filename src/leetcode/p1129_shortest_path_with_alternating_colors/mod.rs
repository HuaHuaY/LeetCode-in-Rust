pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        fn bfs(nodes: &[(Vec<usize>, Vec<usize>)], is_red: bool) -> Vec<usize> {
            let mut red_visit = vec![false; nodes.len()];
            let mut blue_visit = vec![false; nodes.len()];
            let mut min_steps = vec![nodes.len() * 2; nodes.len()];

            let mut queue = VecDeque::new();
            queue.push_back((0, 0, is_red));
            while !queue.is_empty() {
                let (idx, steps, is_red) = queue.pop_front().unwrap();
                min_steps[idx] = min_steps[idx].min(steps);
                if is_red {
                    red_visit[idx] = true;
                    for sons in &nodes[idx].1 {
                        if !blue_visit[*sons] {
                            queue.push_back((*sons, steps + 1, false));
                        }
                    }
                } else {
                    blue_visit[idx] = true;
                    for sons in &nodes[idx].0 {
                        if !red_visit[*sons] {
                            queue.push_back((*sons, steps + 1, true));
                        }
                    }
                }
            }

            min_steps
        }

        let mut nodes = vec![(Vec::new(), Vec::new()); n as usize];
        for edge in red_edges {
            nodes[edge[0] as usize].0.push(edge[1] as usize);
        }
        for edge in blue_edges {
            nodes[edge[0] as usize].1.push(edge[1] as usize);
        }
        let red = bfs(&nodes, true);
        let blue = bfs(&nodes, false);
        red.into_iter()
            .zip(blue)
            .map(|(r, b)| {
                let min = r.min(b);
                if min == nodes.len() * 2 {
                    -1
                } else {
                    min as i32
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::shortest_alternating_paths(3, [[0, 1], [1, 2]].to_vec_vec(), vec![]),
            [0, 1, -1]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::shortest_alternating_paths(3, [[0, 1]].to_vec_vec(), [[2, 1]].to_vec_vec()),
            [0, 1, -1]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::shortest_alternating_paths(
                5,
                [[0, 1], [1, 2], [2, 3], [3, 4]].to_vec_vec(),
                [[1, 2], [2, 3], [3, 1]].to_vec_vec()
            ),
            [0, 1, 2, 3, 7]
        );
    }
}
