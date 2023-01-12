pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let knowledge_map = knowledge
            .into_iter()
            .map(|vec| (vec[0].clone(), vec[1].clone()))
            .collect::<HashMap<_, _>>();

        let mut key = String::new();
        let mut in_key = false;
        s.chars().fold(String::new(), |mut result, c| {
            match c {
                '(' => in_key = true,
                ')' => {
                    result.push_str(
                        knowledge_map
                            .get(&key)
                            .map(|str| str.as_str())
                            .unwrap_or("?"),
                    );
                    key.clear();
                    in_key = false;
                }
                _ => {
                    if in_key {
                        key.push(c);
                    } else {
                        result.push(c);
                    }
                }
            }
            result
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::common::ToVecVecString;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::evaluate(
                "(name)is(age)yearsold".to_string(),
                [["name", "bob"], ["age", "two"]].to_vec_vec_string()
            ),
            "bobistwoyearsold"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::evaluate("hi(name)".to_string(), [["a", "b"]].to_vec_vec_string()),
            "hi?"
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::evaluate(
                "(a)(a)(a)aaa".to_string(),
                [["a", "yes"]].to_vec_vec_string()
            ),
            "yesyesyesaaa"
        );
    }
}
