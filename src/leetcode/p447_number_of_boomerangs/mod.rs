pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut map = HashMap::with_capacity(points.len());
        for p in &points {
            for q in &points {
                let d = (p[0] - q[0]).pow(2) + (p[1] - q[1]).pow(2);
                let count = map.entry(d).or_insert(0);
                result += *count * 2;
                *count += 1;
            }
            map.clear();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::number_of_boomerangs([[0, 0], [1, 0], [2, 0]].to_vec_vec()),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::number_of_boomerangs([[1, 1], [2, 2], [3, 3]].to_vec_vec()),
            2
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::number_of_boomerangs([[1, 1]].to_vec_vec()), 0);
    }
}
