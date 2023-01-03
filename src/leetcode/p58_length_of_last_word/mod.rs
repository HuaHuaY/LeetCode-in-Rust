pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let t = s
            .split(' ')
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();
        t[(t.len() - 1) as usize].len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::length_of_last_word(String::from("Hello World")),
            5
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::length_of_last_word(String::from("   fly me   to   the moon  ")),
            4
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::length_of_last_word(String::from("luffy is still joyboy")),
            6
        );
    }
}
