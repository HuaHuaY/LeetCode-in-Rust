pub struct Solution;

impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        if word1.len() < word2.len() {
            return 0;
        }

        let mut map = word2.bytes().fold([0; 26], |mut acc, b| {
            acc[(b - b'a') as usize] += 1;
            acc
        });
        let mut count = map.iter().filter(|&&i| i != 0).count();

        let mut i = 0;
        let bytes = word1
            .bytes()
            .map(|b| (b - b'a') as usize)
            .collect::<Vec<_>>();
        bytes.iter().fold(0, |acc, &b| {
            map[b] -= 1;
            if map[b] == 0 {
                count -= 1;
            }
            while count == 0 {
                if map[bytes[i]] == 0 {
                    count += 1;
                }
                map[bytes[i]] += 1;
                i += 1;
            }
            acc + i as i64
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::valid_substring_count("bcca".to_string(), "abc".to_string()),
            1
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::valid_substring_count("abcabc".to_string(), "abc".to_string()),
            10
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::valid_substring_count("abcabc".to_string(), "aaabc".to_string()),
            0
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::valid_substring_count("bbbbbb".to_string(), "b".to_string()),
            21
        );
    }
}
