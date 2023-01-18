pub struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(
            nums: &[i32],
            visited: &mut [bool],
            path: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if path.len() == nums.len() {
                result.push(path.clone());
                return;
            }
            for i in 0..nums.len() {
                if visited[i] {
                    continue;
                }
                visited[i] = true;
                path.push(nums[i]);
                dfs(nums, visited, path, result);
                path.pop();
                visited[i] = false;
            }
        }
        let mut result = vec![];
        let mut visited = vec![false; nums.len()];
        let mut path = vec![];
        dfs(&nums, &mut visited, &mut path, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
            [
                [1, 2, 3],
                [1, 3, 2],
                [2, 1, 3],
                [2, 3, 1],
                [3, 1, 2],
                [3, 2, 1]
            ]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::permute(vec![0, 1]), [[0, 1], [1, 0]]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::permute(vec![1]), [[1]]);
    }
}
