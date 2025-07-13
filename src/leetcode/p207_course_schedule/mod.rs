pub struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut in_count =
            prerequisites
                .iter()
                .fold(vec![0; num_courses as usize], |mut arr, v| {
                    arr[v[1] as usize] += 1;
                    arr
                });
        let edges = prerequisites
            .iter()
            .fold(vec![vec![]; num_courses as usize], |mut arr, v| {
                arr[v[0] as usize].push(v[1] as usize);
                arr
            });

        let mut zero_in_nodes = in_count
            .iter()
            .enumerate()
            .filter(|(_, &c)| c == 0)
            .map(|(idx, _)| idx)
            .collect::<Vec<_>>();

        while let Some(node) = zero_in_nodes.pop() {
            for &to in &edges[node] {
                in_count[to] -= 1;
                if in_count[to] == 0 {
                    zero_in_nodes.push(to);
                }
            }
        }

        !in_count.into_iter().any(|i| i != 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert!(Solution::can_finish(2, [[1, 0]].to_vec_vec()));
    }

    #[test]
    fn test2() {
        assert!(!Solution::can_finish(2, [[1, 0], [0, 1]].to_vec_vec()));
    }
}
