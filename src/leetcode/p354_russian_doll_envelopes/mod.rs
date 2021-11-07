pub struct Solution {}

impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1]).reverse()));

        let mut array = vec![0; envelopes.len() + 1];
        let mut length = 1;

        array[1] = envelopes[0][1];
        for i in 1..envelopes.len() {
            if envelopes[i][1] > array[length] {
                length += 1;
                array[length] = envelopes[i][1];
            } else {
                let mut left = 1;
                let mut right = length;
                let mut mid;
                while left <= right {
                    mid = (right - left) / 2 + left;
                    if envelopes[i][1] > array[mid] {
                        left = mid + 1;
                    } else {
                        right = mid - 1;
                    }
                }
                array[left] = envelopes[i][1];
            }
        }
        length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_envelopes(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]]),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_envelopes(vec![vec![1, 1], vec![1, 1], vec![1, 1]]),
            1
        );
    }
}
