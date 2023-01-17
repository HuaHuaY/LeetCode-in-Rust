pub struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let s = s.bytes().map(|b| (b - b'a') as usize).collect::<Vec<_>>();
        let p = p.bytes().map(|b| (b - b'a') as usize).collect::<Vec<_>>();
        let p_length = p.len();
        if s.len() < p_length {
            return vec![];
        }
        
        let mut map = [0; 26];
        let mut result = Vec::new();

        for i in 0..p_length {
            map[s[i]] += 1;
            map[p[i]] -= 1;
        }

        let mut diff = map.iter().filter(|e| **e != 0).count();
        if diff == 0 {
            result.push(0);
        }

        for i in p_length..s.len() {
            let a = s[i - p_length];
            if map[a] == 0 {
                diff += 1;
            }
            map[a] -= 1;
            if map[a] == 0 {
                diff -= 1;
            }

            let b = s[i];
            if map[b] == 0 {
                diff += 1;
            }
            map[b] += 1;
            if map[b] == 0 {
                diff -= 1;
            }

            if diff == 0 {
                result.push((i - p_length + 1) as i32);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string()),
            [0, 6]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_anagrams("abab".to_string(), "ab".to_string()),
            [0, 1, 2]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::find_anagrams("baa".to_string(), "aa".to_string()),
            [1]
        );
    }
}
