pub struct Solution;

impl Solution {
    pub fn capitalize_title(title: String) -> String {
        title
            .split(' ')
            .map(|s| {
                if s.len() <= 2 {
                    s.to_lowercase()
                } else {
                    s[0..1].to_uppercase() + &s[1..].to_lowercase()
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::capitalize_title("capiTalIze tHe titLe".to_string()),
            "Capitalize The Title"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::capitalize_title("First leTTeR of EACH Word".to_string()),
            "First Letter of Each Word"
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::capitalize_title("i lOve leetcode".to_string()),
            "i Love Leetcode"
        );
    }
}
