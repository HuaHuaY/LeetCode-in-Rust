pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        while left < right {
            let area = height[left].min(height[right]) * (right - left) as i32;
            max = max.max(area);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_area([1, 8, 6, 2, 5, 4, 8, 3, 7].to_vec()), 49);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_area([1, 1].to_vec()), 1);
    }
}
