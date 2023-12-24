pub struct Solution;

impl Solution {
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        fn dfs(
            cache: &mut [Vec<Vec<i32>>],
            i: usize,
            j: usize,
            pre_sum: &[i32],
            k: usize,
            t: usize,
        ) -> i32 {
            if cache[i][j][t] != -1 {
                return cache[i][j][t];
            }

            if t > j - i + 1 {
                cache[i][j][t] = i32::MAX;
                return i32::MAX;
            }

            if t == 1 {
                let remain = dfs(cache, i, j, pre_sum, k, k);
                if remain != i32::MAX {
                    cache[i][j][1] = remain
                        + if i == 0 {
                            pre_sum[j]
                        } else {
                            pre_sum[j] - pre_sum[i - 1]
                        };
                    return cache[i][j][1];
                } else {
                    cache[i][j][1] = i32::MAX;
                    return i32::MAX;
                }
            }

            let mut result = i32::MAX;
            for p in (i..j).step_by(k - 1) {
                let left = dfs(cache, i, p, pre_sum, k, 1);
                let right = dfs(cache, p + 1, j, pre_sum, k, t - 1);
                if left != i32::MAX && right != i32::MAX {
                    result = result.min(left + right);
                }
            }
            cache[i][j][t] = result;
            result
        }

        if (stones.len() as i32 - 1) % (k - 1) != 0 {
            return -1;
        }

        let mut cache = vec![vec![vec![-1; k as usize + 1]; stones.len()]; stones.len()];
        let mut pre_sum = vec![0; stones.len()];
        let mut sum = 0;
        for (i, stone) in stones.iter().enumerate() {
            sum += stone;
            pre_sum[i] = sum;
            cache[i][i][1] = 0;
        }
        dfs(&mut cache, 0, stones.len() - 1, &pre_sum, k as usize, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::merge_stones([3, 2, 4, 1].to_vec(), 2), 20);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::merge_stones([3, 2, 4, 1].to_vec(), 3), -1);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::merge_stones([3, 5, 1, 2, 6].to_vec(), 3), 25);
    }
}
