pub struct Solution {}

impl Solution {
    pub fn radix_sort(nums: &mut Vec<i32>) {
        let mut tmp = nums.clone();
        let mut rank = [[0; 256]; 4];
        for i in nums.iter() {
            rank[0][*i as usize & 255] += 1;
            rank[1][(*i >> 8) as usize & 255] += 1;
            rank[2][(*i >> 16) as usize & 255] += 1;
            rank[3][(*i as usize >> 24) & 255] += 1;
        }
        for i in 1..=255 {
            rank[0][i] += rank[0][i - 1];
            rank[1][i] += rank[1][i - 1];
            rank[2][i] += rank[2][i - 1];
            rank[3][i] += rank[3][i - 1];
        }
        for i in (0..nums.len()).rev() {
            rank[0][nums[i] as usize & 255] -= 1;
            tmp[rank[0][nums[i] as usize & 255]] = nums[i];
        }
        for i in (0..nums.len()).rev() {
            rank[1][(tmp[i] as usize >> 8) & 255] -= 1;
            nums[rank[1][(tmp[i] as usize >> 8) & 255]] = tmp[i];
        }
        for i in (0..nums.len()).rev() {
            rank[2][(nums[i] as usize >> 16) & 255] -= 1;
            tmp[rank[2][(nums[i] as usize >> 16) & 255]] = nums[i];
        }
        for i in (0..nums.len()).rev() {
            rank[3][(tmp[i] as usize >> 24) & 255] -= 1;
            nums[rank[3][(tmp[i] as usize >> 24) & 255]] = tmp[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = vec![2, 4, 1, 3, 5];
        Solution::radix_sort(&mut nums);
        assert_eq!(vec![1, 2, 3, 4, 5], nums);
    }

    #[test]
    fn test2() {
        let mut nums = vec![1, 2, 0, 0, 2];
        Solution::radix_sort(&mut nums);
        assert_eq!(vec![0, 0, 1, 2, 2], nums);
    }

    #[test]
    fn test3() {
        let mut nums: Vec<i32> = vec![];
        Solution::radix_sort(&mut nums);
        assert_eq!(vec![] as Vec<i32>, nums);
    }

    #[test]
    fn test4() {
        let mut nums = vec![0];
        Solution::radix_sort(&mut nums);
        assert_eq!(vec![0], nums);
    }
}
