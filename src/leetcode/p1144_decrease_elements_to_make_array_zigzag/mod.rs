pub struct Solution;

impl Solution {
    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        let min_step = |init: usize| {
            let mut min_step = 0;
            for i in (init..nums.len()).step_by(2) {
                let mut min = i32::MAX;
                if i >= 1 {
                    min = min.min(nums[i - 1]);
                }
                if i + 1 < nums.len() {
                    min = min.min(nums[i + 1]);
                }
                if nums[i] > min - 1 {
                    min_step += nums[i] - min + 1;
                }
            }
            min_step
        };
        min_step(0).min(min_step(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::moves_to_make_zigzag([1, 2, 3].to_vec()), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::moves_to_make_zigzag([9, 6, 1, 6, 2].to_vec()), 4);
    }
}
