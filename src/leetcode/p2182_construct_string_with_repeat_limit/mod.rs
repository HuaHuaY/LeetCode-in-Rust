pub struct Solution;

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut statistics = [0; 26];
        for b in s.bytes() {
            statistics[(b - b'a') as usize] += 1;
        }

        let mut result = String::with_capacity(s.len());
        let mut max: i8 = 25;
        let mut max_2: i8 = 24;
        let mut count = 0;

        while max >= 0 && max_2 >= 0 {
            if statistics[max as usize] == 0 {
                max -= 1;
                count = 0;
            } else if count < repeat_limit {
                statistics[max as usize] -= 1;
                result.push((max as u8 + b'a') as char);
                count += 1;
            } else if max_2 >= max || statistics[max_2 as usize] == 0 {
                max_2 -= 1;
            } else {
                statistics[max_2 as usize] -= 1;
                result.push((max_2 as u8 + b'a') as char);
                count = 0;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::repeat_limited_string("cczazcc".to_string(), 3),
            "zzcccac"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::repeat_limited_string("aababab".to_string(), 2),
            "bbabaa"
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::repeat_limited_string("robnsdvpuxbapuqgopqvxdrchivlifeepy".to_string(), 2),
            "yxxvvuvusrrqqppopponliihgfeeddcbba"
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::repeat_limited_string("zyyxx".to_string(), 1),
            "zyxyx"
        );
    }
}
