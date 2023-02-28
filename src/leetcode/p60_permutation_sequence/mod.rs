pub struct Solution;

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n = n as usize;
        let mut k = k as usize;
        let mut factorial = vec![1; n];
        for i in 2..n {
            factorial[i] = factorial[i - 1] * i;
        }

        let mut result = String::with_capacity(n);
        let mut valid = [true; 9];
        for i in 1..=n {
            let index = (k - 1) / factorial[n - i];
            let num = (1..=n).filter(|i| valid[*i - 1]).nth(index).unwrap();
            valid[num - 1] = false;
            result.push((b'0' + num as u8) as char);
            k = (k - 1) % factorial[n - i] + 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::get_permutation(3, 3), "213");
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::get_permutation(4, 9), "2314");
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::get_permutation(3, 1), "123");
    }
}
