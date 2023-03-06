pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 1;
        let mut count = 1;
        while j < nums.len() {
            if nums[j] == nums[j - 1] {
                count += 1;
            } else {
                count = 1;
            }
            if count > 2 {
                j += 1;
                continue;
            }
            i += 1;
            nums[i] = nums[j];
            j += 1;
        }
        i as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut vec = [1, 1, 1, 2, 2, 3].to_vec();
        assert_eq!(Solution::remove_duplicates(&mut vec), 5);
        assert_eq!(vec[..5], [1, 1, 2, 2, 3])
    }

    #[test]
    fn test2() {
        let mut vec = [0, 0, 1, 1, 1, 1, 2, 3, 3].to_vec();
        assert_eq!(Solution::remove_duplicates(&mut vec), 7);
        assert_eq!(vec[..7], [0, 0, 1, 1, 2, 3, 3])
    }
}
