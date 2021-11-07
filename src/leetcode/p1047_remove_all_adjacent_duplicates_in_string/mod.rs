pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = Vec::with_capacity(s.len());
        for i in s.chars() {
            if stack.len() > 0 && i == *stack.last().unwrap() {
                stack.pop();
            } else {
                stack.push(i);
            }
        }
        stack.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::remove_duplicates(String::from("abbaca")), "ca");
    }
}
