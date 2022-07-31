pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let mut stack: Vec<char> = Vec::with_capacity(s.len());

        for c in s {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => {
                    if stack.last() == Some(&'(') {
                        stack.pop();
                    } else {
                        stack.push(c);
                        break;
                    }
                }
                ']' => {
                    if stack.last() == Some(&'[') {
                        stack.pop();
                    } else {
                        stack.push(c);
                        break;
                    }
                }
                '}' => {
                    if stack.last() == Some(&'{') {
                        stack.pop();
                    } else {
                        stack.push(c);
                        break;
                    }
                }
                _ => continue,
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_valid(String::from("()")));
    }

    #[test]
    fn test2() {
        assert!(Solution::is_valid(String::from("()[]{}")));
    }

    #[test]
    fn test3() {
        assert!(!Solution::is_valid(String::from("(]")));
    }

    #[test]
    fn test4() {
        assert!(!Solution::is_valid(String::from("([)]")));
    }

    #[test]
    fn test5() {
        assert!(Solution::is_valid(String::from("{[]}")));
    }
}
