pub struct Solution;

impl Solution {
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let mut res = Vec::with_capacity(word.len());
        let mut sum = 0;
        for b in word.into_bytes() {
            sum = (sum * 10 + (b - b'0') as i64) % m as i64;
            res.push(if sum == 0 { 1 } else { 0 });
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::divisibility_array("998244353".to_string(), 3),
            [1, 1, 0, 0, 0, 1, 1, 0, 0]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::divisibility_array("1010".to_string(), 10),
            [0, 1, 0, 1]
        );
    }
}
