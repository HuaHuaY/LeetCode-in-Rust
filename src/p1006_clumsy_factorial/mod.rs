pub struct Solution {}

impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        let mut n = n;
        let mut stack = Vec::with_capacity(n as usize);
        stack.push(n);
        for i in 0..n - 1 {
            n -= 1;
            match i % 4 {
                0 => {
                    let top = stack.pop().unwrap();
                    stack.push(top * n);
                }
                1 => {
                    let top = stack.pop().unwrap();
                    stack.push(top / n);
                }
                2 => stack.push(n),
                _ => stack.push(-n),
            }
        }
        stack.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::clumsy(4), 7);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::clumsy(10), 12);
    }
}
