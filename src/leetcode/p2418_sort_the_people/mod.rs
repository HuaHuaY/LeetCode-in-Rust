pub struct Solution;

impl Solution {
    pub fn sort_people(mut names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut indices = (0..names.len()).collect::<Vec<_>>();
        indices.sort_unstable_by_key(|i| heights[*i]);
        indices
            .into_iter()
            .rev()
            .map(|i| std::mem::take(&mut names[i]))
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
            Solution::sort_people(
                ["Mary", "John", "Emma"].to_vec_string(),
                [180, 165, 170].to_vec()
            ),
            ["Mary", "Emma", "John"]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::sort_people(
                ["Alice", "Bob", "Bob"].to_vec_string(),
                [155, 185, 150].to_vec()
            ),
            ["Bob", "Alice", "Bob"]
        );
    }
}
