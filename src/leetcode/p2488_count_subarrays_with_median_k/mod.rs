pub struct Solution;

use std::cmp::Ordering;
use std::collections::HashMap;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut pre_sum = HashMap::new();
        pre_sum.insert(0, 1);
        let mut pre = 0;
        let mut bool = false;
        let mut answer = 0;
        for num in nums {
            pre += match num.cmp(&k) {
                Ordering::Greater => 1,
                Ordering::Less => -1,
                Ordering::Equal => {
                    bool = true;
                    0
                }
            };
            if bool {
                answer += *pre_sum.get(&pre).unwrap_or(&0);
                answer += *pre_sum.get(&(pre - 1)).unwrap_or(&0);
            } else {
                pre_sum.entry(pre).and_modify(|e| *e += 1).or_insert(1);
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_subarrays([3, 2, 1, 4, 5].to_vec(), 4), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count_subarrays([2, 3, 1].to_vec(), 3), 1);
    }
}
