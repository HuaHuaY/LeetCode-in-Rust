pub struct Solution;

impl Solution {
    pub fn seconds_to_remove_occurrences(s: String) -> i32 {
        let mut max = 0;
        let mut count_zero = 0;
        for byte in s.as_bytes() {
            match byte {
                b'0' => count_zero += 1,
                b'1' => {
                    max = if count_zero == 0 {
                        0
                    } else {
                        (max + 1).max(count_zero)
                    }
                }
                _ => unreachable!(),
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::seconds_to_remove_occurrences("0110101".to_string()),
            4
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::seconds_to_remove_occurrences("11100".to_string()),
            0
        );
    }
}
