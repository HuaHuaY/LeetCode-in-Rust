pub struct Solution {}

impl Solution {
    pub fn selection_sort(nums: &mut [i32]) {
        for i in 0..nums.len() {
            let mut min = i;
            for j in i + 1..nums.len() {
                if nums[j] < nums[min] {
                    min = j;
                }
            }
            nums.swap(i, min);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = [2, 4, 1, 3, 5];
        Solution::selection_sort(&mut nums);
        assert_eq!(nums, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test2() {
        let mut nums = [1, 2, 0, 0, 2];
        Solution::selection_sort(&mut nums);
        assert_eq!(nums, [0, 0, 1, 2, 2]);
    }

    #[test]
    fn test3() {
        let mut nums: [i32; 0] = [];
        Solution::selection_sort(&mut nums);
        assert_eq!(nums, []);
    }

    #[test]
    fn test4() {
        let mut nums = [0];
        Solution::selection_sort(&mut nums);
        assert_eq!(nums, [0]);
    }
}
