pub struct Solution;

impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, time: i32) -> i32 {
        let mut right_most = vec![0; time as usize + 1];
        for clip in clips {
            if clip[0] < time {
                right_most[clip[0] as usize] = right_most[clip[0] as usize].max(clip[1] as usize);
            }
        }
        let mut result = 0;
        let mut pre = 0;
        let mut max = 0;
        for (i, right) in right_most.into_iter().take(time as usize).enumerate() {
            max = max.max(right);
            if i == max {
                return -1;
            }
            if i == pre {
                result += 1;
                pre = max;
            }
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
            Solution::video_stitching(
                [[0, 2], [4, 6], [8, 10], [1, 9], [1, 5], [5, 9]].to_vec_vec(),
                10
            ),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::video_stitching([[0, 1], [1, 2]].to_vec_vec(), 5),
            -1
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::video_stitching(
                [
                    [0, 1],
                    [6, 8],
                    [0, 2],
                    [5, 6],
                    [0, 4],
                    [0, 3],
                    [6, 7],
                    [1, 3],
                    [4, 7],
                    [1, 4],
                    [2, 5],
                    [2, 6],
                    [3, 4],
                    [4, 5],
                    [5, 7],
                    [6, 9]
                ]
                .to_vec_vec(),
                9
            ),
            3
        );
    }
}
