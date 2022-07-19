pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut alpha = vec![false; 128];
        let mut queue = Vec::with_capacity(s.len());
        let mut result = 0;

        for i in s.bytes() {
            let i = i as usize;
            if alpha[i] {
                while *queue.last().unwrap() != i {
                    alpha[queue.pop().unwrap()] = false;
                }
                queue.pop();
                queue.insert(0, i);
            } else {
                queue.insert(0, i);
                alpha[i] = true;
                result = result.max(queue.len());
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbbb")),
            1
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        );
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::length_of_longest_substring(String::from("")), 0);
    }
}
