pub struct Solution;

#[allow(dead_code)]
impl Solution {
    #[allow(non_snake_case)]
    pub fn hammingWeight(n: u32) -> i32 {
        Self::foo1(n)
    }

    fn foo1(n: u32) -> i32 {
        let mut count = 0;
        let mut n = n;
        while n != 0 {
            n = n & (n - 1);
            count += 1;
        }
        count
    }

    fn foo2(n: u32) -> i32 {
        n.count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::hammingWeight(0b00000000000000000000000000001011),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::hammingWeight(0b00000000000000000000000010000000),
            1
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::hammingWeight(0b11111111111111111111111111111101),
            31
        );
    }
}
