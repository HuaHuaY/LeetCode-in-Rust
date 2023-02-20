pub struct Solution;

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        intervals.push(new_interval);
        for i in (0..intervals.len() - 1).rev() {
            if intervals[i] > intervals[i + 1] {
                intervals.swap(i, i + 1);
            } else {
                break;
            }
        }
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
            Solution::insert([[1, 3], [6, 9]].to_vec_vec(), [2, 5].to_vec()),
            [[1, 5], [6, 9]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::insert(
                [[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]].to_vec_vec(),
                [4, 8].to_vec()
            ),
            [[1, 2], [3, 10], [12, 16]]
        );
    }
}
