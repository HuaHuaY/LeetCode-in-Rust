pub struct Solution;

impl Solution {
    pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
        let mut prefix = vec![0; max_heights.len()];
        let mut suffix = prefix.clone();

        let mut stack = Vec::with_capacity(max_heights.len());
        for i in 0..max_heights.len() {
            while let Some(j) = stack.last() {
                if max_heights[*j] > max_heights[i] {
                    stack.pop();
                } else {
                    break;
                }
            }
            if stack.is_empty() {
                prefix[i] = max_heights[i] as i64 * (i + 1) as i64;
            } else {
                let top = *stack.last().unwrap();
                prefix[i] = prefix[top] + max_heights[i] as i64 * (i - top) as i64;
            }
            stack.push(i);
        }

        stack.clear();
        for i in (0..max_heights.len()).rev() {
            while let Some(j) = stack.last() {
                if max_heights[*j] > max_heights[i] {
                    stack.pop();
                } else {
                    break;
                }
            }
            if stack.is_empty() {
                suffix[i] = max_heights[i] as i64 * (max_heights.len() - i) as i64;
            } else {
                let top = *stack.last().unwrap();
                suffix[i] = suffix[top] + max_heights[i] as i64 * (top - i) as i64;
            }
            stack.push(i);
        }

        prefix
            .into_iter()
            .zip(suffix)
            .zip(max_heights)
            .map(|((i, j), k)| i - k as i64 + j)
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::maximum_sum_of_heights([5, 3, 4, 1, 1].to_vec()),
            13
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::maximum_sum_of_heights([6, 5, 3, 9, 2, 7].to_vec()),
            22
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::maximum_sum_of_heights([3, 2, 5, 5, 2, 3].to_vec()),
            18
        );
    }
}
