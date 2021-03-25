pub struct Solution {}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut answer = vec![vec![0; n as usize]; n as usize];
        let mut i = 0i32;
        let mut j = 0i32;
        let mut direction = Direction::Right;

        for k in 1..=n * n {
            answer[i as usize][j as usize] = k as i32;
            match direction {
                Direction::Up => {
                    if i - 1 < 0 || answer[i as usize - 1][j as usize] != 0 {
                        direction = Direction::Right;
                        j += 1;
                    } else {
                        i -= 1;
                    }
                }
                Direction::Down => {
                    if i + 1 >= n as i32 || answer[i as usize + 1][j as usize] != 0 {
                        direction = Direction::Left;
                        j -= 1;
                    } else {
                        i += 1;
                    }
                }
                Direction::Left => {
                    if j - 1 < 0 || answer[i as usize][j as usize - 1] != 0 {
                        direction = Direction::Up;
                        i -= 1;
                    } else {
                        j -= 1;
                    }
                }
                Direction::Right => {
                    if j + 1 >= n as i32 || answer[i as usize][j as usize + 1] != 0 {
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
            Solution::generate_matrix(3),
            [[1, 2, 3], [8, 9, 4], [7, 6, 5]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::generate_matrix(1), [[1]]);
    }
}
