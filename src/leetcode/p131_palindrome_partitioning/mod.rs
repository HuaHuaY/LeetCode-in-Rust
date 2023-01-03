pub struct Solution;

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
    use crate::common::ToSortVecVec;

    #[test]
    fn test1() {
        let test = Solution::partition(String::from("aab")).to_sort_vec_vec();
        let answer = [vec!["a", "a", "b"], vec!["aa", "b"]].to_sort_vec_vec();
        assert_eq!(test, answer);
    }

    #[test]
    fn test2() {
        let test = Solution::partition(String::from("a")).to_sort_vec_vec();
        let answer = [vec!["a"]].to_sort_vec_vec();
        assert_eq!(test, answer);
    }
}
