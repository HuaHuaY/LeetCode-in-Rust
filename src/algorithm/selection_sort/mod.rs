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
        assert_eq!([1, 2, 3, 4, 5], nums);
    }

    #[test]
    fn test2() {
        let mut nums = [1, 2, 0, 0, 2];
        Solution::selection_sort(&mut nums);
        assert_eq!([0, 0, 1, 2, 2], nums);
    }
}
