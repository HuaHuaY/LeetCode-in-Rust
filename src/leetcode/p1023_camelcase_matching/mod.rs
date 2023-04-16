pub struct Solution;

impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let mut result = Vec::with_capacity(queries.len());
        let pattern = pattern.as_bytes();
        for query in queries.iter().map(|str| str.as_bytes()) {
            let mut i = 0;
            let mut j = 0;
            while i < query.len() && j < pattern.len() {
                if query[i] == pattern[j] {
                    i += 1;
                    j += 1;
                } else if query[i] >= b'A' && query[i] <= b'Z' {
                    break;
                } else {
                    i += 1;
                }
            }
            if query[i..].iter().all(|c| !(&b'A'..=&b'Z').contains(&c)) && j == pattern.len() {
                result.push(true);
            } else {
                result.push(false);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecString;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::camel_match(
                [
                    "FooBar",
                    "FooBarTest",
                    "FootBall",
                    "FrameBuffer",
                    "ForceFeedBack"
                ]
                .to_vec_string(),
                "FB".to_string()
            ),
            [true, false, true, true, false]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::camel_match(
                [
                    "FooBar",
                    "FooBarTest",
                    "FootBall",
                    "FrameBuffer",
                    "ForceFeedBack"
                ]
                .to_vec_string(),
                "FoBa".to_string()
            ),
            [true, false, true, false, false]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::camel_match(
                [
                    "FooBar",
                    "FooBarTest",
                    "FootBall",
                    "FrameBuffer",
                    "ForceFeedBack"
                ]
                .to_vec_string(),
                "FoBaT".to_string()
            ),
            [false, true, false, false, false]
        );
    }
}
