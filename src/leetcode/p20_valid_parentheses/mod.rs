pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let length = s.len();
        let mut stack: Vec<char> = Vec::with_capacity(length);

        for i in 0..length {
            match s[i] {
                '(' | '[' | '{' => stack.push(s[i]),
                ')' => {
                    if stack.last() == Some(&'(') {
                        stack.pop();
                    } else {
                        stack.push(s[i]);
                        break;
                    }
                }
                ']' => {
                    if stack.last() == Some(&'[') {
                        stack.pop();
                    } else {
                        stack.push(s[i]);
                        break;
                    }
                }
                '}' => {
                    if stack.last() == Some(&'{') {
                        stack.pop();
                    } else {
                        stack.push(s[i]);
                        break;
                    }
                }
                _ => continue,
            }
        }
        if stack.len() == 0 {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_valid(String::from("()")), true);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::is_valid(String::from("(]")), false);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::is_valid(String::from("([)]")), false);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::is_valid(String::from("{[]}")), true);
    }
}
