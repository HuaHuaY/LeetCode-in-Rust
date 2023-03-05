pub struct Solution;

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut left_b = vec![0; s.len() + 1];
        let mut right_a = vec![0; s.len() + 1];
        for (i, b) in s.bytes().enumerate() {
            left_b[i + 1] = left_b[i] + if b == b'b' { 1 } else { 0 };
        }
        for (i, b) in s.bytes().enumerate().rev() {
            right_a[i] = right_a[i + 1] + if b == b'a' { 1 } else { 0 };
        }
        left_b
            .into_iter()
            .zip(right_a.into_iter())
            .map(|(b, a)| b + a)
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_deletions("aababbab".to_string()), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::minimum_deletions("bbaaaaabb".to_string()), 2);
    }
}
