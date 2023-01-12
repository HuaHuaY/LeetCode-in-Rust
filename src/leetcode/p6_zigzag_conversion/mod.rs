pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let s = s.chars().collect::<Vec<_>>();
        let num_rows = num_rows as usize;
        let length = s.len();

        let mut str = String::new();
        for i in 0..num_rows {
            let mut j = i;
            let step = 2 * (num_rows - i - 1);
            while j < length {
                str.push(s[j]);
                if step > 0 && step < 2 * (num_rows - 1) && j + step < length {
                    str.push(s[j + step]);
                }
                j += 2 * (num_rows - 1);
            }
        }

        str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI"
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::convert("A".to_string(), 1), "A");
    }
}
