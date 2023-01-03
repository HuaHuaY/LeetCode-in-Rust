pub struct Solution;

impl Solution {
    pub fn shell_sort(nums: &mut [i32]) {
        let mut gap = nums.len() / 2;
        while gap > 0 {
            for i in gap..nums.len() {
                let mut p = i;
                while p >= gap && nums[p] < nums[p - gap] {
                    nums.swap(p, p - gap);
                    p -= gap;
                }
            }
            gap /= 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = [2, 4, 1, 3, 5];
        Solution::shell_sort(&mut nums);
        assert_eq!(nums, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test2() {
        let mut nums = [1, 2, 0, 0, 2];
        Solution::shell_sort(&mut nums);
        assert_eq!(nums, [0, 0, 1, 2, 2]);
    }

    #[test]
    fn test3() {
        let mut nums: [i32; 0] = [];
        Solution::shell_sort(&mut nums);
        assert_eq!(nums, []);
    }

    #[test]
    fn test4() {
        let mut nums = [0];
        Solution::shell_sort(&mut nums);
        assert_eq!(nums, [0]);
    }
}
