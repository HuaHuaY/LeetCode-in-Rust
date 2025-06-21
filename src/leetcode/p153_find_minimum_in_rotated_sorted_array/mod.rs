pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let left = 0;
        let right = nums.len() - 1;
        let mut i = left as i32;
        let mut j = right as i32;
        if nums[i as usize] >= nums[j as usize] {
            while i <= j {
                let mid = ((j - i) / 2 + i) as usize;
                if nums[mid] >= nums[left] {
                    i = mid as i32 + 1;
                } else if nums[mid] <= nums[right] {
                    j = mid as i32 - 1;
                }
            }
        }
        nums[i as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_min([3, 4, 5, 1, 2].to_vec()), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_min([4, 5, 6, 7, 0, 1, 2].to_vec()), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::find_min([11, 13, 15, 17].to_vec()), 11);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::find_min([1].to_vec()), 1);
    }
}
