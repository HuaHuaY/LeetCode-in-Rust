pub struct Solution;

impl Solution {
    pub fn find_lex_smallest_string(mut s: String, a: i32, b: i32) -> String {
        let len = s.len();
        let mut result = s.clone();
        s = s + &result;

        let min_str = |str: &mut [u8], index: usize| {
            let mut min = 10;
            let mut times = 0;
            let mut visited = [false; 10];
            for i in 0..10 {
                let num = ((str[index] - b'0') + i * a as u8) % 10;
                if visited[num as usize] {
                    break;
                } else {
                    visited[num as usize] = true;
                }
                if num < min {
                    min = num;
                    times = i;
                }
            }
            for i in (index..str.len()).step_by(2) {
                str[i] = b'0' + ((str[i] - b'0') + times * a as u8) % 10;
            }
        };

        let mut visited = vec![false; len];
        let mut i = 0;
        while !visited[i] {
            visited[i] = true;
            let mut str = s[i..i + len].as_bytes().to_vec();
            min_str(&mut str, 1);
            if b % 2 == 1 {
                min_str(&mut str, 0);
            }
            result = result.min(str.into_iter().map(|b| b as char).collect::<String>());
            i = (i + b as usize) % len;
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
            Solution::find_lex_smallest_string("5525".to_string(), 9, 2),
            "2050"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_lex_smallest_string("74".to_string(), 5, 1),
            "24"
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::find_lex_smallest_string("0011".to_string(), 4, 2),
            "0011"
        );
    }
}
