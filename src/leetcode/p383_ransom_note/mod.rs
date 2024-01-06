pub struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if magazine.len() < ransom_note.len() {
            return false;
        }

        let mut count = [0; 26];
        ransom_note
            .bytes()
            .for_each(|b| count[(b - b'a') as usize] += 1);
        magazine
            .bytes()
            .for_each(|b| count[(b - b'a') as usize] -= 1);

        count.iter().all(|&c| c <= 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(!Solution::can_construct("a".to_string(), "b".to_string()));
    }

    #[test]
    fn test2() {
        assert!(!Solution::can_construct("aa".to_string(), "ab".to_string()));
    }

    #[test]
    fn test3() {
        assert!(Solution::can_construct("aa".to_string(), "aab".to_string()));
    }
}
