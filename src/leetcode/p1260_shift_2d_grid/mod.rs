pub struct Solution {}

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let inner_len = grid.get(0).unwrap_or(&vec![]).len();
        let mut result: Vec<i32> = IntoIterator::into_iter(grid).flatten().collect();
        let size = result.len();
        result.rotate_right(k as usize % size);
        result.chunks(inner_len).map(|e| e.to_vec()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::*;

    #[test]
    fn test1() {
        let grid = [[1, 2, 3], [4, 5, 6], [7, 8, 9]].to_vec_vec();
        let answer = [[9, 1, 2], [3, 4, 5], [6, 7, 8]].to_vec_vec();
        assert_eq!(Solution::shift_grid(grid, 1), answer);
    }

    #[test]
    fn test2() {
        let grid = [[3, 8, 1, 9], [19, 7, 2, 5], [4, 6, 11, 10], [12, 0, 21, 13]].to_vec_vec();
        let answer = [[12, 0, 21, 13], [3, 8, 1, 9], [19, 7, 2, 5], [4, 6, 11, 10]].to_vec_vec();
        assert_eq!(Solution::shift_grid(grid, 4), answer);
    }

    #[test]
    fn test3() {
        let grid = [[1, 2, 3], [4, 5, 6], [7, 8, 9]].to_vec_vec();
        let answer = [[1, 2, 3], [4, 5, 6], [7, 8, 9]].to_vec_vec();
        assert_eq!(Solution::shift_grid(grid, 9), answer);
    }

    #[test]
    fn test4() {
        let grid = [[1]].to_vec_vec();
        let answer = [[1]].to_vec_vec();
        assert_eq!(Solution::shift_grid(grid, 100), answer);
    }
}
