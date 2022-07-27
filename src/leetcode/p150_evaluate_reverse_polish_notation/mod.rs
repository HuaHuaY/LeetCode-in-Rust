pub struct Solution {}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        for i in tokens {
            match &i[..] {
                "+" => {
                    let top = stack.pop().unwrap();
                    let second = stack.pop().unwrap();
                    stack.push(second + top);
                }
                "-" => {
                    let top = stack.pop().unwrap();
                    let second = stack.pop().unwrap();
                    stack.push(second - top);
                }
                "*" => {
                    let top = stack.pop().unwrap();
                    let second = stack.pop().unwrap();
                    stack.push(second * top);
                }
                "/" => {
                    let top = stack.pop().unwrap();
                    let second = stack.pop().unwrap();
                    stack.push(second / top);
                }
                _ => stack.push(i.trim().parse::<i32>().unwrap()),
            }
        }
        stack[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecString;

    #[test]
    fn test1() {
        let test = ["2", "1", "+", "3", "*"].to_vec_string();
        assert_eq!(Solution::eval_rpn(test), 9);
    }

    #[test]
    fn test2() {
        let test = ["4", "13", "5", "/", "+"].to_vec_string();
        assert_eq!(Solution::eval_rpn(test), 6);
    }

    #[test]
    fn test3() {
        let test = [
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ]
        .to_vec_string();
        assert_eq!(Solution::eval_rpn(test), 22);
    }
}
