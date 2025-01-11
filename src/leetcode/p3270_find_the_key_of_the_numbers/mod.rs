pub struct Solution;

impl Solution {
    pub fn generate_key(mut num1: i32, mut num2: i32, mut num3: i32) -> i32 {
        let mut result = 0;
        for i in [1, 10, 100, 1000] {
            result += (num1 % 10).min(num2 % 10).min(num3 % 10) * i;
            num1 /= 10;
            num2 /= 10;
            num3 /= 10;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::generate_key(1, 10, 1000), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::generate_key(987, 879, 798), 777);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::generate_key(1, 2, 3), 1);
    }
}
