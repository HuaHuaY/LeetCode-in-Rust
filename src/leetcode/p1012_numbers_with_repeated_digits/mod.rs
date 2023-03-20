pub struct Solution;

impl Solution {
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        fn dfs(mask: usize, str: &[u8], i: usize, same: bool) -> i32 {
            if i == str.len() {
                return 1;
            }
            let max_digit = if same { str[i] - b'0' } else { 9 };
            let mut result = 0;
            for j in 0..=max_digit as usize {
                if mask & (1 << j) != 0 {
                    continue;
                }
                if same && j == max_digit as usize {
                    result += dfs(mask | (1 << j), str, i + 1, true);
                } else if mask == 0 && j == 0 {
                    result += dfs(0, str, i + 1, false);
                } else {
                    let mut res = 1;
                    let t = mask.count_ones() as i32 + 1;
                    for i in 10 - t - (str.len() as i32 - i as i32 - 1) + 1..=10 - t {
                        res *= i;
                    }
                    result += res;
                }
            }
            result
        }
        let s = n.to_string();
        n + 1 - dfs(0, s.as_bytes(), 0, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::num_dup_digits_at_most_n(20), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::num_dup_digits_at_most_n(100), 10);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::num_dup_digits_at_most_n(1000), 262);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::num_dup_digits_at_most_n(11), 1);
    }
}
