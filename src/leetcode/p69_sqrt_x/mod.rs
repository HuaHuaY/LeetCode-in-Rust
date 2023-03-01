pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let result = ((x as f64).ln() / 2.0).exp() as i64;
        if (result + 1) * (result + 1) <= x as i64 {
            result as i32 + 1
        } else {
            result as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::my_sqrt(4), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::my_sqrt(8), 2);
    }
}
