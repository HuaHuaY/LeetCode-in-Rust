pub struct Solution;

impl Solution {
    pub fn longest_decomposition(text: String) -> i32 {
        let text = text.as_bytes();
        let len = text.len();
        let mut result = 0;
        let mut i = 0;
        let mut j = len as i32 - 1;
        while i <= j {
            let mut k = 0;
            while i + k < j - k {
                if text[i as usize..=(i + k) as usize] == text[(j - k) as usize..=j as usize] {
                    result += 2;
                    break;
                }
                k += 1;
            }
            if i + k >= j - k {
                result += 1;
            }
            i += k + 1;
            j -= k + 1;
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
            Solution::longest_decomposition("ghiabcdefhelloadamhelloabcdefghi".to_string()),
            7
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::longest_decomposition("merchant".to_string()), 1);
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::longest_decomposition("antaprezatepzapreanta".to_string()),
            11
        );
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::longest_decomposition("a".to_string()), 1);
    }
}
