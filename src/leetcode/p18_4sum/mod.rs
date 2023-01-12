pub struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        nums.sort_unstable();
        for i in 0..nums.len() {
            if i != 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..nums.len() {
                if j != i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }

                let mut k = j + 1;
                let mut l = nums.len() - 1;
                while k < l {
                    let sum = nums[i] as i64 + nums[j] as i64 + nums[k] as i64 + nums[l] as i64;
                    if sum == target as i64 {
                        result.push(vec![nums[i], nums[j], nums[k], nums[l]]);
                    }

                    if sum < target as i64 {
                        k += 1;
                        while k < l && nums[k] == nums[k - 1] {
                            k += 1;
                        }
                    } else {
                        l -= 1;
                        while k < l && nums[l] == nums[l + 1] {
                            l -= 1;
                        }
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
            Solution::four_sum([1, 0, -1, 0, -2, 2].to_vec(), 0),
            [[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::four_sum([2, 2, 2, 2, 2].to_vec(), 8),
            [[2, 2, 2, 2]]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::four_sum(
                [1000000000, 1000000000, 1000000000, 1000000000].to_vec(),
                -294967296
            ),
            [] as [Vec<i32>; 0]
        );
    }
}
