pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        Solution::foo2(nums, val)
    }

    fn foo1(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut j = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[j] = nums[i];
                j += 1;
            }
        }

        j as i32
    }

    fn foo2(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|x| *x != val);
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut test = vec![3, 2, 2, 3];
        assert_eq!(Solution::remove_element(&mut test, 3), 2);

        let mut test = test[..2].to_vec();
        test.sort_unstable();

        let mut answer = [2, 2];
        answer.sort_unstable();

        assert_eq!(test, answer);
    }

    #[test]
    fn test2() {
        let mut test = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(Solution::remove_element(&mut test, 2), 5);

        let mut test = test[..5].to_vec();
        test.sort_unstable();

        let mut answer = [0, 1, 4, 0, 3];
        answer.sort_unstable();

        assert_eq!(test, answer);
    }
}
