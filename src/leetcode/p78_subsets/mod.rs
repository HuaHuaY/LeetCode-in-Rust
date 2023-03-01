pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut result = Vec::with_capacity(1 << len);
        result.push(vec![]);
        for mut i in 1..(1 << len) {
            let mut count = 0;
            let mut tmp = vec![];
            while i != 0 {
                if i & 1 == 1 {
                    tmp.push(nums[count]);
                }
                count += 1;
                i >>= 1;
            }
            result.push(tmp);
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
        let mut vec = Solution::subsets([1, 2, 3].to_vec());
        vec.sort_unstable();
        let mut ans = vec_vec![[], [1], [2], [1, 2], [3], [1, 3], [2, 3], [1, 2, 3]];
        ans.sort_unstable();
        assert_eq!(vec, ans);
    }

    #[test]
    fn test2() {
        let mut vec = Solution::subsets([0].to_vec());
        vec.sort_unstable();
        let mut ans = vec_vec![[], [0]];
        ans.sort_unstable();
        assert_eq!(vec, ans);
    }
}
