pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        nums.sort_unstable();
        for i in 0..nums.len() {
            if i != 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if sum == 0 {
                    result.push(vec![nums[i], nums[j], nums[k]]);
                }

                if sum < 0 {
                    j += 1;
                    while j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                } else {
                    k -= 1;
                    while j < k && nums[k] == nums[k + 1] {
                        k -= 1;
                    }
                }
            }
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
            Solution::three_sum([-1, 0, 1, 2, -1, -4].to_vec()),
            [[-1, -1, 2], [-1, 0, 1]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::three_sum([0, 1, 1].to_vec()), [] as [Vec<i32>; 0]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::three_sum([0, 0, 0].to_vec()), [[0, 0, 0]]);
    }
}
