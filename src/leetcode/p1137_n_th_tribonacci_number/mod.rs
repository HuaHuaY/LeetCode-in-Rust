pub struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        match n {
            0 => 0,
            1 | 2 => 1,
            3.. => {
                let mut a = 0;
                let mut b = 1;
                let mut c = 1;
                for _ in 3..=n {
                    let t = a + b + c;
                    a = b;
                    b = c;
                    c = t;
                }
                c
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
        assert_eq!(Solution::tribonacci(4), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::tribonacci(25), 1389537);
    }
}
