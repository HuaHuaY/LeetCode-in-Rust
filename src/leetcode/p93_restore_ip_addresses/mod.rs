pub struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        fn dfs(n: usize, s: &[char], i: usize, path: &mut Vec<String>, res: &mut Vec<String>) {
            match (n == 5, i == s.len()) {
                (true, true) => {
                    res.push(path.join("."));
                    return;
                }
                (false, false) => (),
                _ => return,
            }
            path.push(s[i].to_string());
            dfs(n + 1, s, i + 1, path, res);
            path.pop();
            if i + 1 < s.len() && s[i] != '0' {
                path.push(s[i..i + 2].iter().collect());
                dfs(n + 1, s, i + 2, path, res);
                path.pop();
            }
            match s[i] {
                '1' if i + 2 < s.len() => {
                    path.push(s[i..i + 3].iter().collect());
                    dfs(n + 1, s, i + 3, path, res);
                    path.pop();
                }
                '2' if i + 2 < s.len() => {
                    if s[i + 1] <= '4' || (s[i + 1] == '5' && s[i + 2] <= '5') {
                        path.push(s[i..i + 3].iter().collect());
                        dfs(n + 1, s, i + 3, path, res);
                        path.pop();
                    }
                }
                _ => {}
            }
        }
        let chars = s.chars().collect::<Vec<_>>();
        let mut result = vec![];
        dfs(1, &chars, 0, &mut Vec::new(), &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::restore_ip_addresses("25525511135".to_string()),
            ["255.255.11.135", "255.255.111.35"]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::restore_ip_addresses("0000".to_string()),
            ["0.0.0.0"]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::restore_ip_addresses("101023".to_string()),
            [
                "1.0.10.23",
                "1.0.102.3",
                "10.1.0.23",
                "10.10.2.3",
                "101.0.2.3"
            ]
        );
    }
}
