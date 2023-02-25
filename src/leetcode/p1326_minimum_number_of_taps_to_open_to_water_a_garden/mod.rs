pub struct Solution;

impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let mut right_most = vec![0; n as usize + 1];
        for (i, range) in ranges.into_iter().enumerate() {
            let left = 0.max(i as i32 - range) as usize;
            let right = (i as i32 + range) as usize;
            right_most[left] = right_most[left].max(right);
        }
        let mut result = 0;
        let mut pre = 0;
        let mut max = 0;
        for (i, right) in right_most.into_iter().take(n as usize).enumerate() {
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

    #[test]
    fn test1() {
        assert_eq!(Solution::min_taps(5, [3, 4, 1, 1, 0, 0].to_vec()), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_taps(3, [0, 0, 0, 0].to_vec()), -1);
    }
}
