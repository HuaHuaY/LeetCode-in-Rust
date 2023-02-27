pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let words = words
            .into_iter()
            .map(|str| str.bytes().map(|b| (b - b'a') as usize).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let values = words
            .iter()
            .map(|idxes| idxes.iter().map(|idx| score[*idx]).sum::<i32>())
            .collect::<Vec<_>>();
        let letters = letters.into_iter().fold(vec![0; 26], |mut vec, letter| {
            vec[(letter as u8 - b'a') as usize] += 1;
            vec
        });

        let mut map = HashMap::new();
        map.insert([0; 26], 0);
        let mut max = 0;
        for (idx, word) in words.into_iter().enumerate() {
            let count = word.into_iter().fold([0; 26], |mut arr, idx| {
                arr[idx] += 1;
                arr
            });
            for (mut k, v) in map.clone() {
                k.iter_mut().zip(count.iter()).for_each(|(k, c)| *k += c);
                if k.iter()
                    .enumerate()
                    .all(|(idx, letter)| *letter <= letters[idx])
                {
                    max = max.max(v + values[idx]);
                    map.insert(k, v + values[idx]);
                }
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{ToVecChar, ToVecString};

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_score_words(
                ["dog", "cat", "dad", "good"].to_vec_string(),
                ["a", "a", "c", "d", "d", "d", "g", "o", "o"].to_vec_char(),
                [1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                    .to_vec()
            ),
            23
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_score_words(
                ["xxxz", "ax", "bx", "cx"].to_vec_string(),
                ["z", "a", "b", "c", "x", "x", "x"].to_vec_char(),
                [4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10]
                    .to_vec()
            ),
            27
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::max_score_words(
                ["leetcode"].to_vec_string(),
                ["l", "e", "t", "c", "o", "d"].to_vec_char(),
                [0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]
                    .to_vec()
            ),
            0
        );
    }
}
