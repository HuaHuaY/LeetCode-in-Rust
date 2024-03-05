pub struct Solution;

impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = [0; 32];
        for mut num in nums {
            while num != 0 {
                let bit = num.trailing_zeros() as usize;
                result[bit] += 1;
                num ^= 1 << bit;
            }
        }
        IntoIterator::into_iter(result)
            .enumerate()
            .filter(|(_, e)| *e >= k)
            .map(|(i, _)| 1 << i)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_k_or([7, 12, 9, 8, 9, 15].to_vec(), 4), 9);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_k_or([2, 12, 1, 11, 4, 5].to_vec(), 6), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::find_k_or([10, 8, 5, 9, 11, 6, 8].to_vec(), 1), 15);
    }
}
