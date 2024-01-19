pub struct Solution;

impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        words
            .iter()
            .flat_map(|word| word.split(separator).filter(|str| !str.is_empty()))
            .map(ToString::to_string)
            .collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecString;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::split_words_by_separator(
                ["one.two.three", "four.five", "six"].to_vec_string(),
                ".".chars().next().unwrap()
            ),
            ["one", "two", "three", "four", "five", "six"]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::split_words_by_separator(
                ["$easy$", "$problem$"].to_vec_string(),
                "$".chars().next().unwrap()
            ),
            ["easy", "problem"]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::split_words_by_separator(
                ["|||"].to_vec_string(),
                "|".chars().next().unwrap()
            ),
            [] as [String; 0]
        );
    }
}
