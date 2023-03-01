pub struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut vec_word = vec![vec![words[0].clone()]];
        let mut vec_len = vec![words[0].len()];
        for word in words.into_iter().skip(1) {
            if *vec_len.last().unwrap() + word.len() < max_width as usize {
                *vec_len.last_mut().unwrap() += word.len() + 1;
                vec_word.last_mut().unwrap().push(word);
            } else {
                vec_len.push(word.len());
                vec_word.push(vec![word]);
            }
        }

        let mut tail = vec_word.pop().unwrap().join(" ");
        let tail_len = tail.len();
        tail += &" ".repeat(max_width as usize - tail_len);
        vec_word
            .into_iter()
            .enumerate()
            .map(|(idx, mut vec)| {
                if vec.len() == 1 {
                    vec.pop().unwrap() + &" ".repeat(max_width as usize - vec_len[idx])
                } else {
                    let gap = (max_width as usize - vec_len[idx]) / (vec.len() - 1) + 1;
                    let mut mod_gap = (max_width as usize - vec_len[idx]) % (vec.len() - 1);
                    let tail = vec.pop().unwrap();
                    let mut result = String::with_capacity(max_width as usize);
                    for word in vec {
                        result += &word;
                        result += &" ".repeat(gap);
                        if mod_gap > 0 {
                            mod_gap -= 1;
                            result += " ";
                        }
                    }
                    result += &tail;
                    result
                }
            })
            .chain(std::iter::once(tail))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecString;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::full_justify(
                [
                    "This",
                    "is",
                    "an",
                    "example",
                    "of",
                    "text",
                    "justification."
                ]
                .to_vec_string(),
                16
            ),
            ["This    is    an", "example  of text", "justification.  "]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::full_justify(
                ["What", "must", "be", "acknowledgment", "shall", "be"].to_vec_string(),
                16
            ),
            ["What   must   be", "acknowledgment  ", "shall be        "]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::full_justify(
                [
                    "Science",
                    "is",
                    "what",
                    "we",
                    "understand",
                    "well",
                    "enough",
                    "to",
                    "explain",
                    "to",
                    "a",
                    "computer.",
                    "Art",
                    "is",
                    "everything",
                    "else",
                    "we",
                    "do"
                ]
                .to_vec_string(),
                20
            ),
            [
                "Science  is  what we",
                "understand      well",
                "enough to explain to",
                "a  computer.  Art is",
                "everything  else  we",
                "do                  "
            ]
        );
    }
}
