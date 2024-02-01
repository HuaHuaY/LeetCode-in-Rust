pub struct Solution;

impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let mut vec = alice_values.into_iter().zip(bob_values).collect::<Vec<_>>();
        vec.sort_unstable_by_key(|(a, b)| -(a + b));
        vec.into_iter()
            .enumerate()
            .map(|(i, (a, b))| if i & 1 == 0 { a } else { -b })
            .sum::<i32>()
            .signum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::stone_game_vi([1, 3].to_vec(), [2, 1].to_vec()), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::stone_game_vi([1, 2].to_vec(), [3, 1].to_vec()), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::stone_game_vi([2, 4, 3].to_vec(), [1, 6, 7].to_vec()),
            -1
        );
    }
}
