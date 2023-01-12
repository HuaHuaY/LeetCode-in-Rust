pub struct Solution;

use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Solution::foo2(s, p)
    }

    fn foo1(s: String, p: String) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        let p = p.chars().collect::<Vec<_>>();
        let m = s.len();
        let n = p.len();

        let matches = |i: usize, j: usize| {
            if i == 0 {
                false
            } else if p[j - 1] == '.' {
                true
            } else {
                s[i - 1] == p[j - 1]
            }
        };

        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;
        for i in 0..=m {
            for j in 1..=n {
                if p[j - 1] == '*' {
                    dp[i][j] = dp[i][j - 2];
                    if matches(i, j - 1) {
                        dp[i][j] |= dp[i - 1][j];
                    }
                } else if matches(i, j) {
                    dp[i][j] = dp[i - 1][j - 1];
                }
            }
        }
        dp[m][n]
    }

    fn foo2(s: String, p: String) -> bool {
        #[derive(Default)]
        struct RegexNode {
            character: char,
            parent: usize,
            children: HashMap<char, Vec<usize>>,
            with_sign: bool,
            end: bool,
        }

        impl RegexNode {
            pub fn new(character: char, parent: usize) -> Self {
                Self {
                    character,
                    parent,
                    ..Default::default()
                }
            }
        }

        let mut nodes = Vec::new();
        let mut last_index = 0;
        nodes.push(RegexNode::default());

        for c in p.chars() {
            match c {
                '*' => {
                    let last_node = &mut nodes[last_index];
                    last_node.with_sign = true;
                    last_node
                        .children
                        .entry(last_node.character)
                        .or_default()
                        .push(last_index);
                }
                _ => {
                    let new_index = nodes.len();
                    nodes.push(RegexNode::new(c, last_index));
                    nodes[last_index]
                        .children
                        .entry(c)
                        .or_default()
                        .push(new_index);
                    while nodes[last_index].with_sign {
                        let last_parent = nodes[last_index].parent;
                        nodes[last_parent]
                            .children
                            .entry(c)
                            .or_default()
                            .push(new_index);
                        last_index = last_parent;
                    }
                    last_index = new_index;
                }
            }
        }

        nodes[last_index].end = true;
        while nodes[last_index].with_sign {
            let last_parent = nodes[last_index].parent;
            nodes[last_parent].end = true;
            last_index = last_parent;
        }

        fn check(nodes: &[RegexNode], now: usize, s: &[char], s_index: usize) -> bool {
            if s_index >= s.len() {
                return nodes[now].end;
            }

            let c = s[s_index];
            let mut children = vec![];
            if let Some(dot) = nodes[now].children.get(&'.') {
                children.extend(dot.clone());
            }
            if let Some(character) = nodes[now].children.get(&c) {
                children.extend(character.clone());
            }

            for child in children {
                if check(nodes, child, s, s_index + 1) {
                    return true;
                }
            }

            false
        }

        let chars = s.chars().collect::<Vec<_>>();
        check(&nodes, 0, &chars, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(!Solution::is_match("aa".to_string(), "a".to_string()));
    }

    #[test]
    fn test2() {
        assert!(Solution::is_match("aa".to_string(), "a*".to_string()));
    }

    #[test]
    fn test3() {
        assert!(Solution::is_match("ab".to_string(), ".*".to_string()));
    }

    #[test]
    fn test4() {
        assert!(Solution::is_match("aab".to_string(), "c*a*b".to_string()));
    }

    #[test]
    fn test5() {
        assert!(Solution::is_match(
            "baabbbaccbccacacc".to_string(),
            "c*..b*a*a.*a..*c".to_string()
        ))
    }
}
