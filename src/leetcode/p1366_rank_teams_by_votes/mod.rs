pub struct Solution;

use std::cmp::Reverse;

impl Solution {
    pub fn rank_teams(votes: Vec<String>) -> String {
        let vec = votes
            .iter()
            .fold(vec![vec![0; votes[0].len()]; 26], |mut vec, str| {
                str.bytes()
                    .map(|b| (b - b'A') as usize)
                    .enumerate()
                    .for_each(|(idx, b)| vec[b][idx] += 1);
                vec
            });

        let mut result = votes[0].as_bytes().to_vec();
        result.sort_unstable_by_key(|b| (Reverse(&vec[(*b - b'A') as usize]), *b));
        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecString;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::rank_teams(["ABC", "ACB", "ABC", "ACB", "ACB"].to_vec_string()),
            "ACB"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::rank_teams(["WXYZ", "XYZW"].to_vec_string()),
            "XWYZ"
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::rank_teams(["ZMNAGUEDSJYLBOPHRQICWFXTVK"].to_vec_string()),
            "ZMNAGUEDSJYLBOPHRQICWFXTVK"
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::rank_teams(["BCA", "CAB", "CBA", "ABC", "ACB", "BAC"].to_vec_string()),
            "ABC"
        );
    }
}
