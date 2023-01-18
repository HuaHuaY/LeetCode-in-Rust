pub struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut s = String::from("1");
        for _ in 1..n {
            let mut t = String::new();
            let mut i = 0;
            while i < s.len() {
                let mut j = i + 1;
                while j < s.len() && s.as_bytes()[j] == s.as_bytes()[i] {
                    j += 1;
                }
                t.push_str(&(j - i).to_string());
                t.push(s.as_bytes()[i] as char);
                i = j;
            }
            s = t;
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_and_say(1), "1");
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count_and_say(4), "1211");
    }
}
