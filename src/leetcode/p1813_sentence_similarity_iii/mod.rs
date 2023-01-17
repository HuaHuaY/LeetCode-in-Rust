pub struct Solution;

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let vec1 = sentence1.split(' ').collect::<Vec<_>>();
        let vec2 = sentence2.split(' ').collect::<Vec<_>>();
        let (vec1, vec2) = if vec1.len() > vec2.len() {
            (vec1, vec2)
        } else {
            (vec2, vec1)
        };

        let len1 = vec1.len();
        let len2 = vec2.len();

        let mut i = 0;
        let mut j = 0;
        while i < len2 && vec1[i] == vec2[i] {
            i += 1;
        }
        while i + j < len2 && vec1[len1 - 1 - j] == vec2[len2 - 1 - j] {
            j += 1;
        }

        i + j == len2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::are_sentences_similar(
            "My name is Haley".to_string(),
            "My Haley".to_string()
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::are_sentences_similar(
            "of".to_string(),
            "A lot of words".to_string()
        ));
    }

    #[test]
    fn test3() {
        assert!(Solution::are_sentences_similar(
            "Eating right now".to_string(),
            "Eating".to_string()
        ));
    }

    #[test]
    fn test4() {
        assert!(Solution::are_sentences_similar(
            "Ogn WtWj HneS".to_string(),
            "Ogn WtWj HneS".to_string()
        ));
    }
}
