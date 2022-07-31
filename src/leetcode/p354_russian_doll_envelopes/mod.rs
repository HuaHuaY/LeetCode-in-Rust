pub struct Solution {}

impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1]).reverse()));

        let mut array = vec![0; envelopes.len() + 1];
        let mut length = 1;

        array[1] = envelopes[0][1];
        for i in envelopes.into_iter().skip(1) {
            if i[1] > array[length] {
                length += 1;
                array[length] = i[1];
            } else {
                let mut left = 1;
                let mut right = length;
                let mut mid;
                while left <= right {
                    mid = (right - left) / 2 + left;
                    if i[1] > array[mid] {
                        left = mid + 1;
                    } else {
                        right = mid - 1;
                    }
                }
                array[left] = i[1];
            }
        }
        length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        let test = [[5, 4], [6, 4], [6, 7], [2, 3]].to_vec_vec();
        assert_eq!(Solution::max_envelopes(test), 3);
    }

    #[test]
    fn test2() {
        let test = [[1, 1], [1, 1], [1, 1]].to_vec_vec();
        assert_eq!(Solution::max_envelopes(test), 1);
    }
}
