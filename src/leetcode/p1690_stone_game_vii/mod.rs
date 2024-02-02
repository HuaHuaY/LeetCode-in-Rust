pub struct Solution;

impl Solution {
    pub fn stone_game_vii(mut stones: Vec<i32>) -> i32 {
        fn dfs(cache: &mut [Vec<i32>], pre_sum: &[i32], left: usize, right: usize) -> i32 {
            if left == right {
                return 0;
            }
            if cache[left][right] != -1 {
                return cache[left][right];
            }
            let mut res = pre_sum[right] - pre_sum[left] - dfs(cache, pre_sum, left + 1, right);
            res = res.max(
                pre_sum[right - 1]
                    - (left != 0).then(|| pre_sum[left - 1]).unwrap_or(0)
                    - dfs(cache, pre_sum, left, right - 1),
            );
            cache[left][right] = res;
            res
        }
        for i in 1..stones.len() {
            stones[i] += stones[i - 1];
        }
        dfs(
            &mut vec![vec![-1; stones.len()]; stones.len()],
            &stones,
            0,
            stones.len() - 1,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::stone_game_vii([5, 3, 1, 4, 2].to_vec()), 6);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::stone_game_vii([7, 90, 5, 1, 100, 10, 10, 2].to_vec()),
            122
        );
    }
}
