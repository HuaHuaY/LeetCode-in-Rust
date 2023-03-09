pub struct Solution;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let len = nums.len();
        let mut result = Vec::with_capacity(1 << len);
        result.push(vec![]);
        for mask in 1..(1 << len) {
            let mut tmp = vec![];
            let mut duplicate = false;
            for (i, n) in nums.iter().enumerate() {
                if mask & (1 << i) != 0 {
                    if i > 0 && mask & (1 << (i - 1)) == 0 && *n == nums[i - 1] {
                        duplicate = true;
                    }
                    tmp.push(*n);
                }
            }
            if !duplicate {
                result.push(tmp);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn test1() {
        let mut nums = Solution::subsets_with_dup([1, 2, 2].to_vec());
        nums.sort_unstable();
        let mut ans = vec_vec![[], [1], [1, 2], [1, 2, 2], [2], [2, 2]];
        ans.sort_unstable();
        assert_eq!(nums, ans);
    }

    #[test]
    fn test2() {
        let mut nums = Solution::subsets_with_dup([0].to_vec());
        nums.sort_unstable();
        let mut ans = vec_vec![[], [0]];
        ans.sort_unstable();
        assert_eq!(nums, ans);
    }
}
