pub struct Solution {}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Solution {
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let size = matrix.len() * matrix[0].len();
        let mut answer = Vec::with_capacity(size);
        let mut i = 0i32;
        let mut j = 0i32;
        let mut direction = Direction::Right;

        while answer.len() != size {
            answer.push(matrix[i as usize][j as usize]);
            matrix[i as usize][j as usize] = -101;
            match direction {
                Direction::Up => {
                    if i - 1 < 0 || matrix[i as usize - 1][j as usize] == -101 {
                        direction = Direction::Right;
                        j += 1;
                    } else {
                        i -= 1;
                    }
                }
                Direction::Down => {
                    if i + 1 >= matrix.len() as i32 || matrix[i as usize + 1][j as usize] == -101 {
                        direction = Direction::Left;
                        j -= 1;
                    } else {
                        i += 1;
                    }
                }
                Direction::Left => {
                    if j - 1 < 0 || matrix[i as usize][j as usize - 1] == -101 {
                        direction = Direction::Up;
                        i -= 1;
                    } else {
                        j -= 1;
                    }
                }
                Direction::Right => {
                    if j + 1 >= matrix[0].len() as i32 || matrix[i as usize][j as usize + 1] == -101 {
                        direction = Direction::Down;
                        i += 1;
                    } else {
                        j += 1;
                    }
                }
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            [1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            [1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }
}
