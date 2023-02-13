pub struct Solution;

impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        fn check(count: &[usize; 4], qlen: usize) -> bool {
            count.iter().all(|a| *a <= qlen)
        }

        let qlen = s.len() >> 2;
        let mut count = [0; 4];
        for c in s.chars() {
            match c {
                'Q' => count[0] += 1,
                'W' => count[1] += 1,
                'E' => count[2] += 1,
                'R' => count[3] += 1,
                _ => (),
            }
        }

        if check(&count, qlen) {
            return 0;
        }

        let mut min_len = i32::MAX;
        let chars = s.chars().collect::<Vec<_>>();
        let mut j = 0;
        for (i, c_front) in chars.iter().enumerate() {
            for c in &chars[j..] {
                if check(&count, qlen) {
                    break;
                }
                match *c {
                    'Q' => count[0] -= 1,
                    'W' => count[1] -= 1,
                    'E' => count[2] -= 1,
                    'R' => count[3] -= 1,
                    _ => (),
                }
                j += 1;
            }

            if !check(&count, qlen) {
                break;
            }

            min_len = min_len.min(j as i32 - i as i32);
            match *c_front {
                'Q' => count[0] += 1,
                'W' => count[1] += 1,
                'E' => count[2] += 1,
                'R' => count[3] += 1,
                _ => (),
            }
        }
        min_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::balanced_string("QWER".to_string()), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::balanced_string("QQWE".to_string()), 1);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::balanced_string("QQQW".to_string()), 2);
    }
}
