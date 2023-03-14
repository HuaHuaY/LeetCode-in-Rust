pub struct Solution;

impl Solution {
    pub fn restore_matrix(row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; col_sum.len()]; row_sum.len()];
        for (i, mut row_sum) in row_sum.into_iter().enumerate() {
            for (j, col_sum) in col_sum.iter_mut().enumerate() {
                let min = row_sum.min(*col_sum);
                result[i][j] = min;
                row_sum -= min;
                *col_sum -= min;
                if row_sum == 0 {
                    break;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let row_sum_ans = [3, 8].to_vec();
        let col_sum_ans = [4, 7].to_vec();
        let mut row_sum = vec![0; row_sum_ans.len()];
        let mut col_sum = vec![0; col_sum_ans.len()];
        for (i, row) in Solution::restore_matrix(row_sum_ans.clone(), col_sum_ans.clone())
            .into_iter()
            .enumerate()
        {
            for (j, col) in row.into_iter().enumerate() {
                row_sum[i] += col;
                col_sum[j] += col;
            }
        }
        assert_eq!(row_sum, row_sum_ans);
        assert_eq!(col_sum, col_sum_ans);
    }

    #[test]
    fn test2() {
        let row_sum_ans = [5, 7, 10].to_vec();
        let col_sum_ans = [8, 6, 8].to_vec();
        let mut row_sum = vec![0; row_sum_ans.len()];
        let mut col_sum = vec![0; col_sum_ans.len()];
        for (i, row) in Solution::restore_matrix(row_sum_ans.clone(), col_sum_ans.clone())
            .into_iter()
            .enumerate()
        {
            for (j, col) in row.into_iter().enumerate() {
                row_sum[i] += col;
                col_sum[j] += col;
            }
        }
        assert_eq!(row_sum, row_sum_ans);
        assert_eq!(col_sum, col_sum_ans);
    }
}
