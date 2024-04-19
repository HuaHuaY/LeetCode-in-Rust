pub struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            2.. => {
                let mut a = 0;
                let mut b = 1;
                for _ in 2..=n {
                    let c = a + b;
                    a = b;
                    b = c;
                }
                b
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::fib(2), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::fib(3), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::fib(4), 3);
    }
}
