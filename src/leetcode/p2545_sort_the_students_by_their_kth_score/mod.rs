pub struct Solution;

impl Solution {
    pub fn sort_the_students(mut score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        score.sort_by_cached_key(|v| -v[k as usize]);
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::sort_the_students(
                [[10, 6, 9, 1], [7, 5, 11, 2], [4, 8, 3, 15]].to_vec_vec(),
                2
            ),
            [[7, 5, 11, 2], [10, 6, 9, 1], [4, 8, 3, 15]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::sort_the_students([[3, 4], [5, 6]].to_vec_vec(), 0),
            [[5, 6], [3, 4]]
        );
    }
}
