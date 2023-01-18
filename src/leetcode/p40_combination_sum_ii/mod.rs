pub struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn dfs(result: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, candidates: &[i32], target: i32) {
            if target < 0 {
                return;
            }
            if target == 0 {
                result.push(path.clone());
                return;
            }

            for i in 0..candidates.len() {
                if i > 0 && candidates[i] == candidates[i - 1] {
                    continue;
                }
                if candidates[i] > target {
                    break;
                }

                path.push(candidates[i]);
                dfs(result, path, &candidates[i + 1..], target - candidates[i]);
                path.pop();
            }
        }

        candidates.sort_unstable();
        let mut result = Vec::new();
        dfs(&mut result, &mut vec![], &candidates, target);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{common::ToSortVecVec, vec_vec};

    #[test]
    fn test1() {
        assert_eq!(
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8).to_sort_vec_vec(),
            vec_vec![[1, 1, 6], [1, 2, 5], [1, 7], [2, 6]].to_sort_vec_vec()
        );
    }
}
