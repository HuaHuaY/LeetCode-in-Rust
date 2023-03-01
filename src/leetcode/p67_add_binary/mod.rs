pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut vec = Vec::with_capacity(a.len().max(b.len()) + 1);
        let mut a = a.bytes().map(|b| b - b'0').rev();
        let mut b = b.bytes().map(|b| b - b'0').rev();
        let mut c = 0;
        loop {
            let sum = match (a.next(), b.next()) {
                (Some(ai), Some(bi)) => ai + bi + c,
                (Some(ai), None) => ai + c,
                (None, Some(bi)) => bi + c,
                (None, None) => break,
            };
            vec.push((sum % 2 + b'0') as char);
            c = sum / 2;
        }
        if c != 0 {
            vec.push((c + b'0') as char);
        }
        vec.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::add_binary("11".to_string(), "1".to_string()),
            "100"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::add_binary("1010".to_string(), "1011".to_string()),
            "10101"
        );
    }
}
