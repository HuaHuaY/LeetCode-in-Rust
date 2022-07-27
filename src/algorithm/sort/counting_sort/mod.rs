pub struct Solution {}

impl Solution {
    pub fn counting_sort(nums: &mut [i32]) {
        if nums.len() == 0 {
            return;
        }

        let min = *nums.iter().min().unwrap();
        let max = *nums.iter().max().unwrap();
        let mut arr = vec![0; (max - min + 1) as usize];
        for n in nums.iter() {
            arr[(*n - min) as usize] += 1;
        }

        let mut index = 0;
        for (i, n) in arr.iter().enumerate() {
            for _ in 0..*n {
                nums[index] = i as i32 + min;
                index += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = [2, 4, 1, 3, 5];
        Solution::counting_sort(&mut nums);
        assert_eq!(nums, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test2() {
        let mut nums = [1, 2, 0, 0, 2];
        Solution::counting_sort(&mut nums);
        assert_eq!(nums, [0, 0, 1, 2, 2]);
    }

    #[test]
    fn test3() {
        let mut nums: [i32; 0] = [];
        Solution::counting_sort(&mut nums);
        assert_eq!(nums, []);
    }

    #[test]
    fn test4() {
        let mut nums = [0];
        Solution::counting_sort(&mut nums);
        assert_eq!(nums, [0]);
    }
}
