pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut alpha = [false; 128];
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
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
    }
}
