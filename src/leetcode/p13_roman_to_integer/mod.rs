pub struct Solution;

use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        Solution::foo1(s)
    }

    fn foo1(s: String) -> i32 {
        let map: HashMap<_, _> = vec![
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
        .into_iter()
        .collect();
        let mut result = 0;
        let chars = s.chars().collect::<Vec<_>>();
        for i in 0..chars.len() {
            if i + 1 < chars.len() && map[&chars[i]] < map[&chars[i + 1]] {
                result -= map[&chars[i]];
            } else {
                result += map[&chars[i]];
            }
        }
        result
    }

    fn foo2(s: String) -> i32 {
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
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
