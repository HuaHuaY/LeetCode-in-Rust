pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let mut s1_cnt = 0;
        let mut s2_cnt = 0;
        let mut index = 0;
        let mut map = HashMap::new();
        let s1_cnt_prime: i32;
        let s2_cnt_prime: i32;
        loop {
            s1_cnt += 1;
            for ch in s1.bytes() {
                if ch == s2.as_bytes()[index] {
                    index += 1;
                    if index == s2.len() {
                        s2_cnt += 1;
                        index = 0;
                    }
                }
            }
            if s1_cnt == n1 {
                return s2_cnt / n2;
            }
            if let Some((s1_prime, s2_prime)) = map.get(&index) {
                s1_cnt_prime = *s1_prime;
                s2_cnt_prime = *s2_prime;
                break;
            } else {
                map.insert(index, (s1_cnt, s2_cnt));
            }
        }
        let mut answer =
            s2_cnt_prime + (n1 - s1_cnt_prime) / (s1_cnt - s1_cnt_prime) * (s2_cnt - s2_cnt_prime);
        for _ in 0..(n1 - s1_cnt_prime) % (s1_cnt - s1_cnt_prime) {
            for ch in s1.bytes() {
                if ch == s2.as_bytes()[index] {
                    index += 1;
                    if index == s2.len() {
                        answer += 1;
                        index = 0;
                    }
                }
            }
        }
        answer / n2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::get_max_repetitions("acb".to_string(), 4, "ab".to_string(), 2),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::get_max_repetitions("acb".to_string(), 1, "acb".to_string(), 1),
            1
        );
    }
}
