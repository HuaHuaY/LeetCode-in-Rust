pub struct Solution;

impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        while i < nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
                i += 2;
            } else {
                i += 1;
            }
        }

        i = 0;
        for j in 0..nums.len() {
            if nums[j] != 0 {
                nums.swap(i, j);
                i += 1;
            }
        }

        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::apply_operations([1, 2, 2, 1, 1, 0].to_vec()),
            [1, 4, 2, 0, 0, 0]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::apply_operations([0, 1].to_vec()), [1, 0]);
    }
}
