pub struct Solution;

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut paths = vec![vec![]; n];
        for road in roads {
            let (a, b, c) = (road[0] as usize, road[1] as usize, road[2]);
            paths[a].push((b, c as i64));
            paths[b].push((a, c as i64));
        }

        let mut min_heap = BinaryHeap::new();
        min_heap.push(Reverse((0, 0, 0)));

        let mut cost = vec![i64::MAX; n];
        let mut count = vec![0i64; n];
        count[0] = 1;

        while let Some(Reverse((c, node, pre))) = min_heap.pop() {
            if c > cost[n - 1] {
                break;
            }
            match c.cmp(&cost[node]) {
                Ordering::Greater => continue,
                Ordering::Equal => {
                    count[node] += count[pre];
                    count[node] %= 1_000_000_007;
                }
                Ordering::Less => {
                    cost[node] = c;
                    count[node] = count[pre];
                    for &(to, cc) in &paths[node] {
                        if cost[to] >= c + cc {
                            min_heap.push(Reverse((c + cc, to, node)));
                            cost[to] = c + cc + 1;
                        }
                    }
                }
            }
        }

        count[n - 1] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_paths(
                7,
                [
                    [0, 6, 7],
                    [0, 1, 2],
                    [1, 2, 3],
                    [1, 3, 3],
                    [6, 3, 3],
                    [3, 5, 1],
                    [6, 5, 1],
                    [2, 5, 1],
                    [0, 4, 5],
                    [4, 6, 2]
                ]
                .to_vec_vec()
            ),
            4
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count_paths(2, [[1, 0, 10]].to_vec_vec()), 1);
    }
}
