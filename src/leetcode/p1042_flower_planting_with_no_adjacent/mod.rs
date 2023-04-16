pub struct Solution;

impl Solution {
    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        let mut path = vec![vec![]; n as usize];
        for p in paths {
            path[p[0] as usize - 1].push(p[1] as usize - 1);
            path[p[1] as usize - 1].push(p[0] as usize - 1);
        }
        let mut answer = vec![0; n as usize];
        for i in 0..n as usize {
            let mut used = vec![false; 5];
            for &j in &path[i] {
                used[answer[j] as usize] = true;
            }
            for j in 1..5 {
                if !used[j] {
                    answer[i] = j as i32;
                    break;
                }
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::garden_no_adj(3, [[1, 2], [2, 3], [3, 1]].to_vec_vec()),
            [1, 2, 3]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::garden_no_adj(4, [[1, 2], [3, 4]].to_vec_vec()),
            [1, 2, 1, 2]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::garden_no_adj(
                4,
                [[1, 2], [2, 3], [3, 4], [4, 1], [1, 3], [2, 4]].to_vec_vec()
            ),
            [1, 2, 3, 4]
        );
    }
}
