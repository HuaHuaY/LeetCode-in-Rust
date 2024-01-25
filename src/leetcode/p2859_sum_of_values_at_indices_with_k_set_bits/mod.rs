pub struct Solution;

impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        if k == 0 {
            return nums[0];
        }

        let mut result = 0;
        let mut i = (1 << k) - 1;
        let len = nums.len();
        while i < len as i32 {
            result += nums[i as usize];
            let lb = i & -i;
            let r = i + lb;
            i = r | ((i ^ r) >> (lb.trailing_zeros() + 2));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::sum_indices_with_k_set_bits([5, 10, 1, 5, 2].to_vec(), 1),
            13
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::sum_indices_with_k_set_bits([4, 3, 2, 1].to_vec(), 2),
            1
        );
    }
}
