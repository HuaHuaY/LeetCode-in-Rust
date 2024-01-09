pub struct Solution;

impl Solution {
    pub fn min_length(s: String) -> i32 {
        s.bytes()
            .fold(Vec::with_capacity(s.len()), |mut stack, b| {
                if let c @ (b'B' | b'D') = b {
                    if let Some(a) = stack.last() {
                        if *a == c - 1 {
                            stack.pop();
                            return stack;
                        }
                    }
                }
                stack.push(b);
                stack
            })
            .len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_length("ABFCACDB".to_string()), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_length("ACBBD".to_string()), 5);
    }
}
