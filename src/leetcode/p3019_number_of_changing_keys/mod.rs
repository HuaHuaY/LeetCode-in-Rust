pub struct Solution;

impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        s.as_bytes()
            .windows(2)
            .filter(|w| !(w[0] as char).eq_ignore_ascii_case(&(w[1] as char)))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_key_changes("aAbBcC".to_string()), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count_key_changes("AaAaAaaA".to_string()), 0);
    }
}
