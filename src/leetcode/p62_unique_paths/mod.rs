pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        (1..m)
            .zip(n..m + n - 1)
            .fold(1, |acc, (x, y)| acc * y as i64 / x as i64) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::unique_paths(36, 7), 4496388);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::unique_paths(23, 12), 193536720);
    }
}
