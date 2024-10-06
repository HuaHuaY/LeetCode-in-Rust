pub struct Solution;

impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let check =
            |min: i64| time.iter().fold(0, |acc, &t| acc + (min / t as i64)) >= total_trips as i64;

        let mut left = 1;
        let mut right = *time.iter().min().unwrap() as i64 * total_trips as i64;
        while left <= right {
            let min = (right - left) / 2 + left;
            if check(min) {
                right = min - 1;
            } else {
                left = min + 1;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_time([1, 2, 3].to_vec(), 5), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::minimum_time([2].to_vec(), 1), 2);
    }
}
