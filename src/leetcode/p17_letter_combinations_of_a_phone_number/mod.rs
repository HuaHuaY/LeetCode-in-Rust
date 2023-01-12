pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let map: HashMap<char, Vec<char>> = vec![
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z']),
        ]
        .into_iter()
        .collect();
        let input = digits
            .chars()
            .map(|c| match c {
                '2'..='9' => map.get(&c).unwrap().clone(),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        let mut result = vec!["".to_string()];
        for chars in input {
            let mut new_result = vec![];
            for s in result {
                for c in chars.iter() {
                    new_result.push(format!("{}{}", s, c));
                }
            }
            result = new_result;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::letter_combinations("".to_string()),
            [] as [String; 0]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::letter_combinations("2".to_string()),
            ["a", "b", "c"]
        );
    }
}
