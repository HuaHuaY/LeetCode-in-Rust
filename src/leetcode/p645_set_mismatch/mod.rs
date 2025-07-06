pub struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len() as i32;

        let xor = (1..=len).chain(nums.iter().cloned()).fold(0, |mut xor, i| {
            xor ^= i;
            xor
        });

        let bit = xor & (-xor);
        let num1 = (1..=len)
            .chain(nums.iter().cloned())
            .filter(|&n| n & bit == 0)
            .fold(0, |xor, i| xor ^ i);
        let num2 = (1..=len)
            .chain(nums.iter().cloned())
            .filter(|&n| n & bit != 0)
            .fold(0, |xor, i| xor ^ i);

        if nums.into_iter().any(|n| n == num1) {
            return vec![num1, num2];
        }
        vec![num2, num1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_error_nums([1, 2, 2, 4].to_vec()), [2, 3]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_error_nums([1, 1].to_vec()), [1, 2]);
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::find_error_nums([3, 2, 3, 4, 6, 5].to_vec()),
            [3, 1]
        );
    }
}
