pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_triplets(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut map = HashMap::new();
        for i in &nums {
            for j in &nums {
                map.entry(*i & *j).and_modify(|e| *e += 1).or_insert(1);
            }
        }
        for (k, v) in map {
            for i in &nums {
                if k & *i == 0 {
                    result += v;
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
        assert_eq!(Solution::count_triplets([2, 1, 3].to_vec()), 12);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count_triplets([0, 0, 0].to_vec()), 27);
    }
}
