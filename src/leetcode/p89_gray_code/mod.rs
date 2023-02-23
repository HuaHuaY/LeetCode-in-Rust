pub struct Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut res = Vec::with_capacity(1 << n);
        res.push(0);
        for i in 1..=n {
            res.extend(res.clone().into_iter().rev().map(|e| e | (1 << (i - 1))));
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::gray_code(2), [0, 1, 3, 2]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::gray_code(1), [0, 1]);
    }
}
