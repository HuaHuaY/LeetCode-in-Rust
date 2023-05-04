pub struct Solution;

impl Solution {
    pub fn hardest_worker(_n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let mut last = 0;
        let mut max = 0;
        let mut id = 0;
        for log in logs {
            let (i, log) = (log[0], log[1]);
            if max < log - last {
                max = log - last;
                id = i;
            } else if max == log - last && id > i {
                id = i;
            }
            last = log;
        }
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::hardest_worker(10, [[0, 3], [2, 5], [0, 9], [1, 15]].to_vec_vec()),
            1
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::hardest_worker(26, [[1, 1], [3, 7], [2, 12], [7, 17]].to_vec_vec()),
            3
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::hardest_worker(2, [[0, 10], [1, 20]].to_vec_vec()),
            0
        );
    }
}
