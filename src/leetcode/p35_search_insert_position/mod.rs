pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;

        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        while left <= right {
            let mid = (right - left) / 2 + left;
            match nums[mid as usize].cmp(&target) {
                Ordering::Greater => right = mid - 1,
                Ordering::Less => left = mid + 1,
                Ordering::Equal => return mid,
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }
}
