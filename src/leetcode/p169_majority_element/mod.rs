pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = 0;
        for num in nums {
            if count == 0 {
                candidate = num;
            }
            if num == candidate {
                count += 1;
            } else {
                count -= 1;
            }
        }
        candidate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::majority_element([3, 2, 3].to_vec()), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::majority_element([2, 2, 1, 1, 1, 2, 2].to_vec()),
            2
        );
    }
}
