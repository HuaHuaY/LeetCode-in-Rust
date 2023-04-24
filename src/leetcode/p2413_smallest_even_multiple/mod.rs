pub struct Solution;

impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        n << (n & 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::smallest_even_multiple(5), 10);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::smallest_even_multiple(6), 6);
    }
}
