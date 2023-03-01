pub struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut c = 1;
        for i in digits.iter_mut().rev() {
            *i += c;
            if *i >= 10 {
                c = 1;
                *i %= 10;
            } else {
                c = 0;
                break;
            }
        }
        if c == 1 {
            digits.insert(0, 1);
        }
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::plus_one([1, 2, 3].to_vec()), [1, 2, 4]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::plus_one([4, 3, 2, 1].to_vec()), [4, 3, 2, 2]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::plus_one([9].to_vec()), [1, 0]);
    }
}
