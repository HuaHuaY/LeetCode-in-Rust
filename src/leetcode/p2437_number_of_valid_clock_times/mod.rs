pub struct Solution;

impl Solution {
    pub fn count_time(time: String) -> i32 {
        let mut count = 1;
        let bytes = time.as_bytes();
        match (bytes[0], bytes[1]) {
            (b'?', b'?') => count *= 24,
            (b'0'..=b'1', b'?') => count *= 10,
            (b'2', b'?') => count *= 4,
            (b'?', b'0'..=b'3') => count *= 3,
            (b'?', b'4'..=b'9') => count *= 2,
            _ => (),
        }
        match (bytes[3], bytes[4]) {
            (b'?', b'?') => count *= 60,
            (b'0'..=b'5', b'?') => count *= 10,
            (b'?', b'0'..=b'9') => count *= 6,
            _ => (),
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_time("?5:00".to_string()), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count_time("0?:0?".to_string()), 100);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::count_time("??:??".to_string()), 1440);
    }
}
