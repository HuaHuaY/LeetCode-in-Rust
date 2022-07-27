pub struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut left = 0;
        let mut right = matrix.len() as i32 - 1;
        let mut mid;
        while left <= right {
            mid = ((right - left) >> 1) + left;
            if matrix[mid as usize][0] == target {
                return true;
            } else if matrix[mid as usize][0] > target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        if right < 0 {
            return false;
        }

        let index = right;
        let mut left = 0;
        let mut right = matrix[index as usize].len() as i32 - 1;
        while left <= right {
            mid = ((right - left) >> 1) + left;
            if matrix[index as usize][mid as usize] == target {
                return true;
            } else if matrix[index as usize][mid as usize] > target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        let test = [[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]].to_vec_vec();
        assert_eq!(Solution::search_matrix(test, 3), true);
    }

    #[test]
    fn test2() {
        let test = [[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]].to_vec_vec();
        assert_eq!(Solution::search_matrix(test, 13), false);
    }
}
