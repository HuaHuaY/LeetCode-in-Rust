pub struct Solution;

impl Solution {
    pub fn add_minimum(word: String) -> i32 {
        let mut cnt = 1;
        let bytes = word.as_bytes();
        for (i, byte) in bytes.iter().enumerate().skip(1) {
            if *byte <= bytes[i - 1] {
                cnt += 1;
            }
        }
        cnt * 3 - word.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::add_minimum("b".to_string()), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::add_minimum("aaa".to_string()), 6);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::add_minimum("abc".to_string()), 0);
    }
}
