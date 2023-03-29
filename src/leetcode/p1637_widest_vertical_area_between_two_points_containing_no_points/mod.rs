pub struct Solution;

impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable();
        points
            .iter()
            .zip(points.iter().skip(1))
            .map(|(a, b)| b[0] - a[0])
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_width_of_vertical_area([[8, 7], [9, 9], [7, 4], [9, 7]].to_vec_vec()),
            1
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_width_of_vertical_area(
                [[3, 1], [9, 0], [1, 0], [1, 4], [5, 3], [8, 8]].to_vec_vec()
            ),
            3
        );
    }
}
