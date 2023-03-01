pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        match n {
            1 => 1,
            2..=45 => {
                let mut a = 1;
                let mut b = 2;
                for _ in 3..=n {
                    b += a;
                    a = b - a;
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
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
