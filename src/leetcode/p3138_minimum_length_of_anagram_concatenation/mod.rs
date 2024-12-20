pub struct Solution;

impl Solution {
    pub fn min_anagram_length(s: String) -> i32 {
        let len = s.len();
        let bytes = s.as_bytes();
        let mut min = 1;
        while min + min <= len {
            if len % min != 0 {
                min += 1;
                continue;
            }
            let first = bytes[0..min].iter().fold([0; 26], |mut set, i| {
                set[(i - b'a') as usize] += 1;
                set
            });
            let mut equal = true;
            for j in (min..len).step_by(min) {
                let next = bytes[j..j + min].iter().fold([0; 26], |mut set, i| {
                    set[(i - b'a') as usize] += 1;
                    set
                });
                if first != next {
                    equal = false;
                    break;
                }
            }
            if equal {
                return min as i32;
            }
            min += 1;
        }
        len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_anagram_length("abba".to_string()), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_anagram_length("cdef".to_string()), 4);
    }
}
