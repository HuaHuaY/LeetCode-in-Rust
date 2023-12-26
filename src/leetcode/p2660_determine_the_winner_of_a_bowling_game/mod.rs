pub struct Solution;

impl Solution {
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        fn calc(player: &[i32]) -> i32 {
            let mut double = 0;
            let mut point = 0;
            for i in player {
                if double != 0 {
                    double -= 1;
                    point += i * 2;
                } else {
                    point += i;
                }
                if *i == 10 {
                    double = 2;
                }
            }
            point
        }
        match calc(&player1).cmp(&calc(&player2)) {
            std::cmp::Ordering::Less => 2,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::is_winner([4, 10, 7, 9].to_vec(), [6, 5, 2, 3].to_vec()),
            1
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::is_winner([3, 5, 7, 6].to_vec(), [8, 10, 10, 2].to_vec()),
            2
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::is_winner([2, 3].to_vec(), [4, 1].to_vec()), 0);
    }
}
