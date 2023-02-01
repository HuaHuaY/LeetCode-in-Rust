pub struct Solution;

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let table = key
            .bytes()
            .filter(|b| *b != b' ')
            .map(|b| (b - b'a') as usize)
            .fold((b'a', [0; 26]), |(mut alphabet, mut arr), idx| {
                if arr[idx] == 0 {
                    arr[idx] = alphabet;
                    alphabet += 1;
                }
                (alphabet, arr)
            })
            .1;
        message
            .bytes()
            .map(|b| {
                if b == b' ' {
                    ' '
                } else {
                    table[(b - b'a') as usize] as char
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::decode_message(
                "the quick brown fox jumps over the lazy dog".to_string(),
                "vkbs bs t suepuv".to_string()
            ),
            "this is a secret"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::decode_message(
                "eljuxhpwnyrdgtqkviszcfmabo".to_string(),
                "zwx hnfx lqantp mnoeius ycgk vcnjrdb".to_string()
            ),
            "the five boxing wizards jump quickly"
        );
    }
}
