pub struct Solution;

impl Solution {
    pub fn base_neg2(mut n: i32) -> String {
        if n == 0 || n == 1 {
            return n.to_string();
        }
        let mut result = vec![];
        while n != 0 {
            let remain = n & 1;
            result.push((remain as u8 + b'0') as char);
            n -= remain;
            n /= -2;
        }
        result.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::base_neg2(2), "110");
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::base_neg2(3), "111");
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::base_neg2(4), "100");
    }
}
