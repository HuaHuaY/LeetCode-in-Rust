pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let word_length = words[0].len();
        let words_length = words.len();
        let mut result = Vec::new();
        for i in 0..word_length {
            let mut map = HashMap::new();
            for word in &words {
                map.entry(word.as_str())
                    .and_modify(|e| *e -= 1)
                    .or_insert(-1);
            }

            let mut diff = map.len();
            let mut count = 0;
            for j in (0 + i..s.len()).step_by(word_length) {
                if j + word_length > s.len() {
                    break;
                }

                if count == words_length {
                    let k = j - word_length * words_length;
                    let v_pop = map.get_mut(&s[k..k + word_length]).unwrap();
                    if *v_pop == 0 {
                        diff += 1;
                    }
                    *v_pop -= 1;
                    if *v_pop == 0 {
                        diff -= 1;
                    }

                    let v_push = map.entry(&s[j..j + word_length]).or_default();
                    if *v_push == 0 {
                        diff += 1;
                    }
                    *v_push += 1;
                    if *v_push == 0 {
                        diff -= 1;
                    }

                    if diff == 0 {
                        result.push(j - (words_length - 1) * word_length);
                    }
                } else {
                    let v = map.entry(&s[j..j + word_length]).or_default();
                    if *v == 0 {
                        diff += 1;
                    }
                    *v += 1;
                    if *v == 0 {
                        diff -= 1;
                    }

                    count += 1;
                    if count == words_length && diff == 0 {
                        result.push(j - (words_length - 1) * word_length)
                    }
                }
            }
        }
        result.into_iter().map(|e| e as i32).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::common::ToVecString;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_substring(
                "barfoothefoobarman".to_string(),
                ["foo", "bar"].to_vec_string()
            ),
            [0, 9]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                ["word", "good", "best", "word"].to_vec_string()
            ),
            [] as [i32; 0]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::find_substring(
                "barfoofoobarthefoobarman".to_string(),
                ["bar", "foo", "the"].to_vec_string()
            ),
            [6, 9, 12]
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                ["word", "good", "best", "good"].to_vec_string()
            ),
            [8]
        );
    }
}
