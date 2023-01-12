pub struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut brackets_stack: Vec<i32> = Vec::with_capacity(s.len());
        brackets_stack.push(1);

        let mut result = 0;
        let mut num = 0;
        let mut opt = 1;
        for (i, n) in s.bytes().enumerate() {
            if n.is_ascii_digit() {
                num = num * 10 + (n - b'0') as i32;
            }

            if !n.is_ascii_digit() || i == s.len() - 1 {
                result += opt * num as i32;
                num = 0;
                match n {
                    b'+' => opt = *brackets_stack.last_mut().unwrap(),
                    b'-' => opt = -*brackets_stack.last_mut().unwrap(),
                    b'(' => brackets_stack.push(opt),
                    b')' => {
                        brackets_stack.pop().unwrap();
                    }
                    _ => {}
                }
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
        assert_eq!(Solution::calculate("1 + 1".to_string()), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::calculate(" 2-1 + 2 ".to_string()), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string()), 23);
    }
}
