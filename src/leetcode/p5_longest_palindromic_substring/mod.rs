pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut str = vec!['#'];
        for c in s.chars() {
            str.push(c);
            str.push('#');
        }

        let mut max_start = 0;
        let mut max_end = 0;
        let mut longest_center = 0;
        let mut longest_length = 0;
        let mut length_vec = Vec::<usize>::with_capacity(str.len());

        for i in 0..str.len() {
            let mut length = 0;
            if i < longest_center + longest_length {
                let mirror = longest_center * 2 - i;
                let mirror_length = length_vec[mirror];
                length = mirror_length.min(longest_center + longest_length - i);
                if mirror - mirror_length != longest_center - longest_length {
                    length_vec.push(length);
                    continue;
                }
            }

            while i > length
                && i + length + 1 < str.len()
                && str[i - length - 1] == str[i + length + 1]
            {
                length += 1;
            }

            length_vec.push(length);

            if i + length > longest_center + longest_length {
                longest_center = i;
                longest_length = length;
            }

            if 2 * longest_length + 1 > max_end - max_start + 1 {
                max_start = longest_center - longest_length;
                max_end = longest_center + longest_length;
            }
        }

        let mut result = String::new();
        for c in str.into_iter().take(max_end + 1).skip(max_start) {
            if c != '#' {
                result.push(c);
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
        assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab");
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
    }
}
