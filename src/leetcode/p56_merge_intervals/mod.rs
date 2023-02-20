pub struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable();
        let mut result = vec![];
        let mut vec = intervals[0].clone();
        for interval in intervals.into_iter().skip(1) {
            if interval[0] <= vec[1] {
                vec[1] = vec[1].max(interval[1]);
            } else {
                result.push(vec);
                vec = interval;
            }
        }
        result.push(vec);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::common::ToVecVec;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::merge([[1, 3], [2, 6], [8, 10], [15, 18]].to_vec_vec()),
            [[1, 6], [8, 10], [15, 18]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::merge([[1, 4], [4, 5]].to_vec_vec()), [[1, 5]]);
    }
}
