pub struct Solution;

impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut prev = 0;
        for num in s.split(' ').filter_map(|str| str.parse::<u8>().ok()) {
            if prev >= num {
                return false;
            }
            prev = num;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::are_numbers_ascending(
            "1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_string()
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::are_numbers_ascending(
            "hello world 5 x 5".to_string()
        ));
    }

    #[test]
    fn test3() {
        assert!(!Solution::are_numbers_ascending(
            "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s".to_string()
        ));
    }
}
