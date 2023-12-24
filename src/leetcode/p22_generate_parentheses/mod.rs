pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 1 {
            return vec!["()".to_string()];
        }

        Solution::generate_parenthesis(n - 1)
            .into_iter()
            .flat_map(|str| {
                let mut result = vec![];
                for i in 0..str.len() {
                    result.push(format!("{}(){}", &str[0..i], &str[i..]));
                }
                result
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::common::ToSortVec;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::generate_parenthesis(3).to_sort_vec(),
            ["((()))", "(()())", "(())()", "()(())", "()()()"].to_sort_vec()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::generate_parenthesis(1), ["()"]);
    }
}
