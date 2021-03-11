pub struct Solution {}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack: Vec<i32> = Vec::with_capacity(s.len());

        let mut num = 0;
        let mut opt = b'+';
        for (i, n) in s.bytes().enumerate() {
            if n.is_ascii_digit() {
                num = num * 10 + (n - b'0') as i32;
            }

            if (!n.is_ascii_digit() && n != b' ') || i == s.len() - 1 {
                match opt {
                    b'+' => stack.push(num),
                    b'-' => stack.push(-num),
                    b'*' => {
                        let tmp = stack.pop().unwrap();
                        stack.push(tmp * num)
                    }
                    b'/' => {
                        let tmp = stack.pop().unwrap();
                        stack.push(tmp / if num == 0 { 1 } else { num })
                    }
                    _ => (),
                }
                opt = n;
                num = 0;
            }
        }
        stack.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::calculate(String::from("3+2*2")), 7);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::calculate(String::from(" 3/2 ")), 1);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::calculate(String::from(" 3+5 / 2 ")), 5);
    }
}
