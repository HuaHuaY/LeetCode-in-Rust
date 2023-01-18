pub struct Solution;

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn dfs(result: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, candidates: &[i32], target: i32) {
            if target < 0 {
                return;
            }
            if target == 0 {
                result.push(path.clone());
                return;
            }

            for i in 0..candidates.len() {
                if candidates[i] > target {
                    break;
                }

                path.push(candidates[i]);
                dfs(result, path, &candidates[i..], target - candidates[i]);
                path.pop();
            }
        }

        candidates.sort();
        let mut result = vec![];
        dfs(&mut result, &mut vec![], &candidates, target);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec_vec![[2, 2, 3], [7]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec_vec![[2, 2, 2, 2], [2, 3, 3], [3, 5]]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::combination_sum(vec![2], 1), [] as [[i32; 0]; 0]);
    }
}
