pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut sum = 0;
        let length = chars.len();
        for i in 0..length {
            sum += match chars[i] {
                'I' => {
                    if i + 1 < length && (chars[i + 1] == 'V' || chars[i + 1] == 'X') {
                        -1
                    } else {
                        1
                    }
                }
                'V' => 5,
                'X' => {
                    if i + 1 < length && (chars[i + 1] == 'L' || chars[i + 1] == 'C') {
                        -10
                    } else {
                        10
                    }
                }
                'L' => 50,
                'C' => {
                    if i + 1 < length && (chars[i + 1] == 'D' || chars[i + 1] == 'M') {
                        -100
                    } else {
                        100
                    }
                }
                'D' => 500,
                'M' => 1000,
                _ => 0,
            };
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::roman_to_int(String::from("IV")), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::roman_to_int(String::from("IX")), 9);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }
}
