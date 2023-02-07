pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        let length = grid.len();
        let status = (0, 0, 0);
        let mut visited = vec![vec![[-1; 2]; length]; length];
        visited[0][0][0] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(status);
        while !queue.is_empty() {
            let (tailx, taily, head) = queue.pop_front().unwrap();

            if head == 0 {
                if tailx < length - 1
                    && grid[tailx + 1][taily] == 0
                    && grid[tailx + 1][taily + 1] == 0
                    && visited[tailx][taily][1] == -1
                {
                    visited[tailx][taily][1] = visited[tailx][taily][0] + 1;
                    queue.push_back((tailx, taily, 1));
                }
                if taily < length - 2
                    && grid[tailx][taily + 2] == 0
                    && visited[tailx][taily + 1][0] == -1
                {
                    visited[tailx][taily + 1][0] = visited[tailx][taily][0] + 1;
                    queue.push_back((tailx, taily + 1, 0));
                }
                if tailx < length - 1
                    && grid[tailx + 1][taily] == 0
                    && grid[tailx + 1][taily + 1] == 0
                    && visited[tailx + 1][taily][0] == -1
                {
                    visited[tailx + 1][taily][0] = visited[tailx][taily][0] + 1;
                    queue.push_back((tailx + 1, taily, 0));
                }
            } else {
                if taily < length - 1
                    && grid[tailx][taily + 1] == 0
                    && grid[tailx + 1][taily + 1] == 0
                    && visited[tailx][taily][0] == -1
                {
                    visited[tailx][taily][0] = visited[tailx][taily][1] + 1;
                    queue.push_back((tailx, taily, 0));
                }
                if tailx < length - 2
                    && grid[tailx + 2][taily] == 0
                    && visited[tailx + 1][taily][1] == -1
                {
                    visited[tailx + 1][taily][1] = visited[tailx][taily][1] + 1;
                    queue.push_back((tailx + 1, taily, 1));
                }
                if taily < length - 1
                    && grid[tailx][taily + 1] == 0
                    && grid[tailx + 1][taily + 1] == 0
                    && visited[tailx][taily + 1][1] == -1
                {
                    visited[tailx][taily + 1][1] = visited[tailx][taily][1] + 1;
                    queue.push_back((tailx, taily + 1, 1));
                }
            }
        }
        visited[length - 1][length - 2][0]
    }
}

#[cfg(test)]
mod tests {
    use crate::common::ToVecVec;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::minimum_moves(
                [
                    [0, 0, 0, 0, 0, 1],
                    [1, 1, 0, 0, 1, 0],
                    [0, 0, 0, 0, 1, 1],
                    [0, 0, 1, 0, 1, 0],
                    [0, 1, 1, 0, 0, 0],
                    [0, 1, 1, 0, 0, 0]
                ]
                .to_vec_vec()
            ),
            11
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::minimum_moves(
                [
                    [0, 0, 1, 1, 1, 1],
                    [0, 0, 0, 0, 1, 1],
                    [1, 1, 0, 0, 0, 1],
                    [1, 1, 1, 0, 0, 1],
                    [1, 1, 1, 0, 0, 1],
                    [1, 1, 1, 0, 0, 0]
                ]
                .to_vec_vec()
            ),
            9
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::minimum_moves(
                [
                    [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
                    [0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0],
                    [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                    [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1],
                    [0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                ]
                .to_vec_vec()
            ),
            -1
        );
    }
}
