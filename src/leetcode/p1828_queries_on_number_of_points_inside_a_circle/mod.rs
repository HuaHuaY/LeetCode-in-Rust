pub struct Solution;

impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        for query in queries {
            let mut count = 0;
            for point in &points {
                let x = point[0] - query[0];
                let y = point[1] - query[1];
                let r = query[2];
                if x * x + y * y <= r * r {
                    count += 1;
                }
            }
            result.push(count);
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
            Solution::count_points(
                [[1, 3], [3, 3], [5, 3], [2, 2]].to_vec_vec(),
                [[2, 3, 1], [4, 3, 1], [1, 1, 2]].to_vec_vec()
            ),
            [3, 2, 2]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::count_points(
                [[1, 1], [2, 2], [3, 3], [4, 4], [5, 5]].to_vec_vec(),
                [[1, 2, 2], [2, 2, 2], [4, 3, 2], [4, 3, 3]].to_vec_vec()
            ),
            [2, 3, 2, 4]
        );
    }
}
