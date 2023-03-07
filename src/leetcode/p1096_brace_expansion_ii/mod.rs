pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        let mut stack = vec![];
        let mut queue = vec![];
        let chars = expression.chars().collect::<Vec<_>>();
        for i in 0..chars.len() {
            match chars[i] {
                ',' => {
                    while let Some(c) = stack.last() {
                        if *c == '*' {
                            stack.pop();
                            queue.push('*');
                        } else {
                            break;
                        }
                    }
                    stack.push('+');
                }
                '{' => {
                    if i > 0 && (chars[i - 1] == '}' || chars[i - 1].is_lowercase()) {
                        stack.push('*');
                    }
                    stack.push('{')
                }
                '}' => {
                    while let Some(c) = stack.pop() {
                        if c == '{' {
                            break;
                        } else {
                            queue.push(c);
                        }
                    }
                }
                'a'..='z' => {
                    if i > 0 && (chars[i - 1] == '}' || chars[i - 1].is_lowercase()) {
                        stack.push('*');
                    }
                    queue.push(chars[i]);
                }
                _ => unreachable!(),
            }
        }
        while let Some(c) = stack.pop() {
            queue.push(c);
        }
        let mut stack: Vec<HashSet<String>> = vec![];
        for c in queue {
            match c {
                '*' => {
                    let t2 = stack.pop().unwrap();
                    let t1 = stack.pop().unwrap();
                    let mut set = HashSet::with_capacity(t1.len() * t2.len());
                    for string1 in t1 {
                        for string2 in &t2 {
                            set.insert(string1.clone() + string2);
                        }
                    }
                    stack.push(set);
                }
                '+' => {
                    let t2 = stack.pop().unwrap();
                    let mut t1 = stack.pop().unwrap();
                    t1.extend(t2);
                    stack.push(t1);
                }
                'a'..='z' => {
                    let mut set = HashSet::new();
                    set.insert(c.to_string());
                    stack.push(set);
                }
                _ => unreachable!(),
            }
        }
        let mut vec = stack.pop().unwrap().into_iter().collect::<Vec<_>>();
        vec.sort_unstable();
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::brace_expansion_ii("{a,b}{c,{d,e}}".to_string()),
            ["ac", "ad", "ae", "bc", "bd", "be"]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::brace_expansion_ii("{{a,z},a{b,c},{ab,z}}".to_string()),
            ["a", "ab", "ac", "z"]
        );
    }
}
