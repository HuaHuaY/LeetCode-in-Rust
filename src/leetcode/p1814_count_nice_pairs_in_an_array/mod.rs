pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();

        let rev = |mut num: i32| {
            let mut rev = 0;
            while num != 0 {
                rev = rev * 10 + num % 10;
                num /= 10;
            }
            rev
        };

        let mut result = 0;
        for num in nums {
            let diff = num - rev(num);
            let v = map.entry(diff).or_insert(0);
            if *v != 0 {
                result += *v;
                result %= 1_000_000_007;
            }
            *v += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_nice_pairs(vec![42, 11, 1, 97]), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count_nice_pairs(vec![13, 10, 35, 24, 76]), 4);
    }
}
