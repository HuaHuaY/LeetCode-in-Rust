pub struct Solution;

struct NumArray {
    pre_sum: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl NumArray {
    fn new(mut nums: Vec<i32>) -> Self {
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        Self { pre_sum: nums }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;
        self.pre_sum[right] - (left >= 1).then(|| self.pre_sum[left - 1]).unwrap_or(0)
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let obj = NumArray::new([-2, 0, 3, -5, 2, -1].to_vec());
        assert_eq!(obj.sum_range(0, 2), 1);
        assert_eq!(obj.sum_range(2, 5), -1);
        assert_eq!(obj.sum_range(0, 5), -3);
    }
}
