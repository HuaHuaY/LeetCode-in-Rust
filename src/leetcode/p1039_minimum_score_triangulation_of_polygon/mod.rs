pub struct Solution;

impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        fn dfs(cache: &mut [Vec<i32>], i: usize, j: usize, values: &[i32]) -> i32 {
            if cache[i][j] != -1 {
                return cache[i][j];
            }
            if j == i + 2 {
                cache[i][j] = values[i] * values[i + 1] * values[j];
                return cache[i][j];
            }
            let mut min = (values[i] * values[i + 1] * values[j] + dfs(cache, i + 1, j, values))
                .min(values[i] * values[j - 1] * values[j] + dfs(cache, i, j - 1, values));
            for k in i + 2..=j - 2 {
                let left = dfs(cache, i, k, values);
                let right = dfs(cache, k, j, values);
                min = min.min(left + right + values[i] * values[k] * values[j]);
            }
            cache[i][j] = min;
            min
        }
        dfs(
            &mut vec![vec![-1; values.len()]; values.len()],
            0,
            values.len() - 1,
            &values,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_score_triangulation([1, 2, 3].to_vec()), 6);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_score_triangulation([3, 7, 4, 5].to_vec()),
            144
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::min_score_triangulation([1, 3, 1, 4, 1, 5].to_vec()),
            13
        );
    }
}
