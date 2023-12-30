pub struct Solution;

const DAYS: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        fn is_leap_year(y: i32) -> bool {
            (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
        }

        let year = date[0..4].parse::<i32>().unwrap();
        let month = date[5..7].parse::<i32>().unwrap();
        let day = date[8..10].parse::<i32>().unwrap();
        (0..month as usize - 1).map(|m| DAYS[m]).sum::<i32>()
            + day
            + if is_leap_year(year) && month > 2 {
                1
            } else {
                0
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::day_of_year("2019-01-09".to_string()), 9);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::day_of_year("2019-02-10".to_string()), 41);
    }
}
