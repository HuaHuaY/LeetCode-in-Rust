pub struct Solution;

use std::collections::HashMap;

#[derive(Default)]
struct TreeNode {
    is_leaf: bool,
    map: HashMap<String, TreeNode>,
}

impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut map = TreeNode::default();

        folder.into_iter().for_each(|s| {
            let mut map_mut = &mut map;
            for s in s.split('/').skip(1) {
                map_mut = map_mut
                    .map
                    .entry(s.to_string())
                    .or_insert(TreeNode::default());
            }
            map_mut.is_leaf = true;
        });

        fn dfs(key: String, value: TreeNode) -> Vec<String> {
            if value.is_leaf {
                return vec![key];
            }

            value
                .map
                .into_iter()
                .flat_map(|(k, v)| dfs(k, v).into_iter().map(|s| format!("{}/{}", key, s)))
                .collect()
        }

        dfs(String::new(), map)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{ToSortVec, ToVecString};

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::remove_subfolders(["/a", "/a/b", "/c/d", "/c/d/e", "/c/f"].to_vec_string())
                .to_sort_vec(),
            ["/a", "/c/d", "/c/f"].to_sort_vec()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::remove_subfolders(["/a", "/a/b/c", "/a/b/d"].to_vec_string()).to_sort_vec(),
            ["/a"].to_sort_vec()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::remove_subfolders(["/a/b/c", "/a/b/ca", "/a/b/d"].to_vec_string())
                .to_sort_vec(),
            ["/a/b/c", "/a/b/ca", "/a/b/d"].to_sort_vec()
        );
    }
}
