pub struct Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let len = position.len();
        let mut position = position.into_iter().zip(speed).fold(
            Vec::with_capacity(len),
            |mut arr, (pos, speed)| {
                arr.push((pos, (target - pos) as f64 / speed as f64));
                arr
            },
        );
        position.sort_unstable_by_key(|&(idx, _)| idx);

        let mut pre_time = position.last().unwrap().1;
        let mut result = 1;
        for (_, time) in position.into_iter().rev().skip(1) {
            if pre_time < time {
                result += 1;
                pre_time = time;
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
        assert_eq!(
            Solution::car_fleet(12, [10, 8, 0, 5, 3].to_vec(), [2, 4, 1, 1, 3].to_vec()),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::car_fleet(10, [3].to_vec(), [3].to_vec()), 1);
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::car_fleet(100, [0, 2, 4].to_vec(), [4, 2, 1].to_vec()),
            1
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::car_fleet(10, [0, 4, 2].to_vec(), [2, 1, 3].to_vec()),
            1
        );
    }
}
