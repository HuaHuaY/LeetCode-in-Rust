pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        Solution::foo2(nums)
    }

    fn foo1(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            0
        } else {
            let mut j = 0;
            for i in 1..nums.len() {
                if nums[i] != nums[j] {
                    j += 1;
                    nums[j] = nums[i];
                }
            }
            j as i32 + 1
        }
    }

    fn foo2(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
        assert_eq!(nums[..2], [1, 2]);
    }

    #[test]
    fn test2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(nums[..5], [0, 1, 2, 3, 4]);
    }
}
