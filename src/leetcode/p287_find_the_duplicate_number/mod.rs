pub struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = nums[0];
        let mut fast = nums[0];

        loop {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];
            if slow == fast {
                break;
            }
        }

        let mut finder = nums[0];
        while finder != slow {
            finder = nums[finder as usize];
            slow = nums[slow as usize];
        }

        finder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_duplicate([1, 3, 4, 2, 2].to_vec()), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_duplicate([3, 1, 3, 4, 2].to_vec()), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::find_duplicate([3, 3, 3, 3, 3].to_vec()), 3);
    }
}
