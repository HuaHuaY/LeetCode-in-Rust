pub struct Solution;

impl Solution {
    pub fn maximum_gain(s: String, mut x: i32, mut y: i32) -> i32 {
        let mut result = 0;

        let mut a = b'a';
        let mut b = b'b';

        if x < y {
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut x, &mut y);
        }

        let mut count_a = 0;
        let mut count_b = 0;

        for byte in s.bytes() {
            if byte == a {
                count_a += 1;
            } else if byte == b {
                if count_a > 0 {
                    count_a -= 1;
                    result += x;
                } else {
                    count_b += 1;
                }
            } else {
                result += count_a.min(count_b) * y;
                count_a = 0;
                count_b = 0;
            }
        }

        result + count_a.min(count_b) * y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::maximum_gain("cdbcbbaaabab".to_string(), 4, 5), 19);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::maximum_gain("aabbaaxybbaabb".to_string(), 5, 4),
            20
        );
    }
}
