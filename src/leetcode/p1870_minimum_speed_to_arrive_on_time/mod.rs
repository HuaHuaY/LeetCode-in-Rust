pub struct Solution;

impl Solution {
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        if hour <= (dist.len() - 1) as f64 {
            return -1;
        }
        let hour = (hour * 100f64).round() as i64;
        let len = dist.len();

        let check = |speed: i64| {
            dist.iter()
                .map(|x| *x as i64)
                .enumerate()
                .fold(0, |acc, (i, x)| {
                    if i != len - 1 && x % speed != 0 {
                        acc + (x / speed + 1) * speed
                    } else {
                        acc + x
                    }
                })
                * 100
                <= hour * speed
        };

        let mut left = 1;
        let mut right = 10000000;
        while left <= right {
            let mid = (right - left) / 2 + left;
            if check(mid as i64) {
                right = mid - 1;
            } else {
                left = mid + 1;
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
        assert_eq!(Solution::min_speed_on_time([1, 3, 2].to_vec(), 6f64), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_speed_on_time([1, 3, 2].to_vec(), 2.7), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::min_speed_on_time([1, 3, 2].to_vec(), 1.9), -1);
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::min_speed_on_time([1, 1, 100000].to_vec(), 2.01),
            10000000
        );
    }
}
