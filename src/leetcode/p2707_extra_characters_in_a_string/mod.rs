pub struct Solution;

struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            children: Default::default(),
            is_end: false,
        }
    }

    fn insert<T: IntoIterator<Item = u8>>(&mut self, s: T) {
        let mut node = self;
        for c in s {
            let idx = (c - b'a') as usize;
            node = node.children[idx].get_or_insert_with(|| Box::new(Trie::new()));
        }
        node.is_end = true;
    }

    fn track(&self, c: u8) -> Option<&Trie> {
        let idx = (c - b'a') as usize;
        self.children[idx].as_deref()
    }
}

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let len = s.len();
        let mut trie = Trie::new();
        for word in dictionary {
            trie.insert(word.bytes().rev());
        }

        let mut dp = vec![0; len + 1];
        for i in 1..=len {
            dp[i] = dp[i - 1] + 1;
            let mut node = &trie;
            for j in (0..i).rev() {
                if let Some(t) = node.track(s.as_bytes()[j]) {
                    if t.is_end {
                        dp[i] = dp[i].min(dp[j]);
                    }
                    node = t;
                } else {
                    break;
                }
            }
        }
        dp[len]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecString;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_extra_char(
                "leetscode".to_string(),
                ["leet", "code", "leetcode"].to_vec_string()
            ),
            1
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_extra_char(
                "sayhelloworld".to_string(),
                ["hello", "world"].to_vec_string()
            ),
            3
        );
    }
}
