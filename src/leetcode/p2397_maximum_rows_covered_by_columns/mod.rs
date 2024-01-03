pub struct Solution;

impl Solution {
    pub fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut mask = Vec::with_capacity(m);
        for row in matrix {
            let mut m = 0;
            for (j, v) in row.into_iter().enumerate() {
                m |= v << j;
            }
            mask.push(m);
        }
        let mut result = 0;
        let mut i = (1 << num_select) - 1;
        while i < (1 << n) {
            let mut t = 0;
            for m in &mask {
                if m & i == *m {
                    t += 1;
                }
            }
            result = result.max(t);
            let c = i & -i;
            let r = i + c;
            i = ((i ^ r) >> (2 + c.trailing_zeros())) | r;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::maximum_rows([[0, 0, 0], [1, 0, 1], [0, 1, 1], [0, 0, 1]].to_vec_vec(), 2),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::maximum_rows([[1], [0]].to_vec_vec(), 1), 2);
    }
}
