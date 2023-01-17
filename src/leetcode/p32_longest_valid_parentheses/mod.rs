pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        Solution::foo1(s)
    }

    fn foo1(s: String) -> i32 {
        use std::cmp::Ordering;

        let mut max = 0;
        let mut left = 0;
        let mut right = 0;

        for c in s.chars() {
            match c {
                '(' => left += 1,
                ')' => right += 1,
                _ => unreachable!(),
            }
            match left.cmp(&right) {
                Ordering::Greater => (),
                Ordering::Equal => max = max.max(left + right),
                Ordering::Less => {
                    left = 0;
                    right = 0;
                }
            }
        }

        left = 0;
        right = 0;
        for c in s.chars().rev() {
            match c {
                '(' => left += 1,
                ')' => right += 1,
                _ => unreachable!(),
            }
            match left.cmp(&right) {
                Ordering::Less => (),
                Ordering::Equal => max = max.max(left + right),
                Ordering::Greater => {
                    left = 0;
                    right = 0;
                }
            }
        }

        max
    }

    fn foo2(s: String) -> i32 {
        let mut max = 0;

        let mut stack = vec![0];
        for c in s.chars() {
            match c {
                '(' => stack.push(0),
                ')' => {
                    if stack.len() == 1 {
                        max = max.max(stack[0]);
                        stack[0] = 0;
                        continue;
                    }
                    let value = stack.pop().unwrap() + 2;
                    *stack.last_mut().unwrap() += value;
                }
                _ => unreachable!(),
            }
        }
        max.max(stack.into_iter().max().unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::longest_valid_parentheses("()(()".to_string()), 2);
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::longest_valid_parentheses("(()(((()".to_string()),
            2
        );
    }
}
