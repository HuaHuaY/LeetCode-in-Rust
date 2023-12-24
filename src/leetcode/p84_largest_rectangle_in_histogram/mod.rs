pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut left = vec![0; heights.len()];
        let mut right = vec![heights.len() - 1; heights.len()];
        let mut stack = vec![];
        for i in 0..heights.len() {
            while let Some(j) = stack.last() {
                if heights[*j] < heights[i] {
                    break;
                }
                right[*j] = i - 1;
                stack.pop();
            }
            left[i] = if let Some(j) = stack.last() {
                *j + 1
            } else {
                0
            };
            stack.push(i);
        }
        heights
            .into_iter()
            .zip(left.into_iter().zip(right))
            .map(|(height, (left, right))| height * (right - left + 1) as i32)
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
            Solution::largest_rectangle_area([2, 1, 5, 6, 2, 3].to_vec()),
            10
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::largest_rectangle_area([2, 4].to_vec()), 4);
    }
}
