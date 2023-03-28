pub struct Solution;

impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut vec = [1; 5];
        for _ in 1..n {
            for j in (0..5).rev() {
                for k in (0..j).rev() {
                    vec[j] += vec[k];
                }
            }
        }
        vec.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_vowel_strings(1), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count_vowel_strings(2), 15);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::count_vowel_strings(33), 66045);
    }
}
