pub struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let arr_len = obstacle_grid[0].len();
        let mut vec1 = vec![0; arr_len];
        let mut vec2 = vec![0; arr_len];
        for (i, n) in obstacle_grid[0].iter().enumerate() {
            if *n == 1 {
                vec1[i] = 0;
            } else if i == 0 {
                vec1[0] = 1;
            } else {
                vec1[i] = vec1[i - 1];
            }
        }
        for row in obstacle_grid.iter().skip(1) {
            for (i, n) in row.iter().enumerate() {
                if *n == 1 {
                    vec2[i] = 0;
                } else if i == 0 {
                    vec2[0] = vec1[0];
                } else {
                    vec2[i] = vec1[i] + vec2[i - 1];
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
            Solution::unique_paths_with_obstacles([[0, 0, 0], [0, 1, 0], [0, 0, 0]].to_vec_vec()),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::unique_paths_with_obstacles([[0, 1], [0, 0]].to_vec_vec()),
            1
        );
    }
}
