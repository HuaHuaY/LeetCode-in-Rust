pub struct Solution;

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let len = matrix[0].len();
        matrix
            .into_iter()
            .reduce(|arr1, mut arr2| {
                for (i, n) in arr2.iter_mut().enumerate() {
                    match i {
                        0 => *n += arr1[i].min(arr1[i + 1]),
                        i if i == len - 1 => *n += arr1[i - 1].min(arr1[i]),
                        _ => *n += arr1[i - 1..=i + 1].iter().min().unwrap(),
                    }
                }
                arr2
            })
            .unwrap()
            .into_iter()
            .min()
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
            Solution::min_falling_path_sum([[2, 1, 3], [6, 5, 4], [7, 8, 9]].to_vec_vec()),
            13
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_falling_path_sum([[-19, 57], [-40, -5]].to_vec_vec()),
            -59
        );
    }
}
