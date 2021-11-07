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

    #[test]
    fn test1() {
        let mut test = Vec::new();
        for i in ["2", "1", "+", "3", "*"].iter() {
            test.push(String::from(*i));
        }
        assert_eq!(Solution::eval_rpn(test), 9);
    }

    #[test]
    fn test2() {
        let mut test = Vec::new();
        for i in ["4", "13", "5", "/", "+"].iter() {
            test.push(String::from(*i));
        }
        assert_eq!(Solution::eval_rpn(test), 6);
    }

    #[test]
    fn test3() {
        let mut test = Vec::new();
        for i in [
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ]
        .iter()
        {
            test.push(String::from(*i));
        }
        assert_eq!(Solution::eval_rpn(test), 22);
    }
}
