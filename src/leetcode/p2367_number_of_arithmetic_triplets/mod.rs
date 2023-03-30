pub struct Solution;

impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut i = 0;
        let mut j = 1;
        let mut k = 2;
        let len = nums.len();
        let mut result = 0;
        while i < len - 2 && j < len - 1 && k < len {
            while j < len - 1 && nums[j] - nums[i] < diff {
                j += 1;
            }
            if j == len - 1 {
                break;
            }
            if nums[j] - nums[i] > diff {
                i += 1;
                continue;
            }
            while k < len && nums[k] - nums[j] < diff {
                k += 1;
            }
            if k == len {
                break;
            }
            if nums[k] - nums[j] > diff {
                i += 1;
                continue;
            }
            result += 1;
            i += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::arithmetic_triplets([0, 1, 4, 6, 7, 10].to_vec(), 3),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::arithmetic_triplets([4, 5, 6, 7, 8, 9].to_vec(), 2),
            2
        );
    }
}
