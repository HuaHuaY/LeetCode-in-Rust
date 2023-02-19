pub struct Solution;

impl Solution {
    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        if suits.iter().all(|suit| *suit == suits[0]) {
            return "Flush".to_string();
        }

        let mut count = [0; 13];
        ranks.into_iter().for_each(|e| count[e as usize - 1] += 1);
        match count.iter().max().unwrap() {
            5 | 4 | 3 => "Three of a Kind".to_string(),
            2 => "Pair".to_string(),
            1 => "High Card".to_string(),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecChar;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::best_hand(
                [13, 2, 3, 1, 9].to_vec(),
                ["a", "a", "a", "a", "a"].to_vec_char()
            ),
            "Flush"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::best_hand(
                [4, 4, 2, 4, 4].to_vec(),
                ["d", "a", "a", "b", "c"].to_vec_char()
            ),
            "Three of a Kind"
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::best_hand(
                [10, 10, 2, 12, 9].to_vec(),
                ["a", "b", "c", "a", "d"].to_vec_char()
            ),
            "Pair"
        );
    }
}
