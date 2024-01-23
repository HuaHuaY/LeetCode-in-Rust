pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut num = num.to_string().into_bytes();
        let mut max = (usize::MAX, u8::MIN);
        let mut last_change = (num.len() - 1, num.len() - 1);
        for (i, num) in num.iter().enumerate().rev() {
            match max.1.cmp(num) {
                Ordering::Greater => last_change = (max.0, i),
                Ordering::Less => max = (i, *num),
                Ordering::Equal => {}
            }
        }
        num.swap(last_change.0, last_change.1);
        num.into_iter()
            .fold(0, |acc, x| acc * 10 + (x - b'0') as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::maximum_swap(2736), 7236);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::maximum_swap(9973), 9973);
    }
}
