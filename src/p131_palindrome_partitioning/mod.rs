pub struct Solution {}

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        struct DFSEnv {
            s: Vec<char>,
            is_palindrome_i_j: Vec<Vec<bool>>,
            answer: Vec<Vec<String>>,
            stack: Vec<String>,
        }

        fn dfs(dfs_env: &mut DFSEnv, i: usize) {
            if i == dfs_env.s.len() {
                dfs_env.answer.push(dfs_env.stack.clone());
            } else {
                for j in i..dfs_env.s.len() {
                    if dfs_env.is_palindrome_i_j[i][j] {
                        dfs_env.stack.push(dfs_env.s[i..=j].iter().collect());
                        dfs(dfs_env, j + 1);
                        dfs_env.stack.pop();
                    }
                }
            }
        }

        let mut dfs_env = DFSEnv {
            is_palindrome_i_j: vec![vec![true; s.len()]; s.len()],
            answer: Vec::new(),
            stack: Vec::new(),
            s: s.chars().collect(),
        };

        for i in (0..dfs_env.s.len()).rev() {
            for j in i + 1..dfs_env.s.len() {
                dfs_env.is_palindrome_i_j[i][j] =
                    (dfs_env.s[i] == dfs_env.s[j]) && dfs_env.is_palindrome_i_j[i + 1][j - 1];
            }
        }

        dfs(&mut dfs_env, 0);

        dfs_env.answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let mut test = Solution::partition(String::from("aab"))
            .into_iter()
            .map(|mut x| {
                x.sort();
                x
            })
            .collect::<Vec<Vec<String>>>();
        test.sort();

        let mut answer = [vec!["a", "a", "b"], vec!["aa", "b"]]
            .to_vec()
            .into_iter()
            .map(|mut x| {
                x.sort();
                x
            })
            .collect::<Vec<Vec<&str>>>();
        answer.sort();

        assert_eq!(test, answer);
    }

    #[test]
    fn test2() {
        let mut test = Solution::partition(String::from("a"))
            .into_iter()
            .map(|mut x| {
                x.sort();
                x
            })
            .collect::<Vec<Vec<String>>>();
        test.sort();

        let mut answer = [vec!["a"]]
            .to_vec()
            .into_iter()
            .map(|mut x| {
                x.sort();
                x
            })
            .collect::<Vec<Vec<&str>>>();
        answer.sort();

        assert_eq!(test, answer);
    }
}
