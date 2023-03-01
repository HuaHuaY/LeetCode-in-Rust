pub struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let arr_len = grid[0].len();
        let mut vec1 = vec![0; arr_len];
        let mut vec2 = vec![0; arr_len];
        for (i, n) in grid[0].iter().enumerate() {
            if i == 0 {
                vec1[0] = *n;
            } else {
                vec1[i] = vec1[i - 1] + *n;
            }
        }
        for row in grid.iter().skip(1) {
            for (i, n) in row.iter().enumerate() {
                if i == 0 {
                    vec2[0] = vec1[0] + *n;
                } else {
                    vec2[i] = vec1[i].min(vec2[i - 1]) + *n;
                }
            }
            std::mem::swap(&mut vec1, &mut vec2);
            vec2.fill(0);
        }
        vec1[arr_len - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_path_sum([[1, 3, 1], [1, 5, 1], [4, 2, 1]].to_vec_vec()),
            7
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_path_sum([[1, 2, 3], [4, 5, 6]].to_vec_vec()),
            12
        );
    }
}
