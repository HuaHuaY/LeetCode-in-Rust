pub struct Solution {}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let length = s.len();
        let string: Vec<char> = s.chars().collect();
        let mut brackets_stack: Vec<i32> = Vec::with_capacity(length);
        brackets_stack.push(1);

        let mut i = 0;
        let mut result = 0;
        let mut opt = 1;
        while i < length {
            match string[i] {
                '+' => opt = *brackets_stack.last_mut().unwrap(),
                '-' => opt = -*brackets_stack.last_mut().unwrap(),
                '(' => brackets_stack.push(opt),
                ')' => {
                    brackets_stack.pop().unwrap();
                }
                '0'..='9' => {
                    let mut num = 0;
                    while i < length {
                        if let Some(y) = string[i].to_digit(10) {
                            num = num * 10 + y;
                            i += 1;
                        } else {
                            i -= 1;
                            break;
                        }
                    }
                    result += opt * num as i32;
                }
                _ => {}
            }
            i += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::calculate(String::from("1 + 1")), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::calculate(String::from(" 2-1 + 2 ")), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::calculate(String::from("(1+(4+5+2)-3)+(6+8)")), 23);
    }
}
