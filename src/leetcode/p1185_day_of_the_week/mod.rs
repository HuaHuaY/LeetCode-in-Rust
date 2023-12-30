pub struct Solution;

const WEEK: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        fn is_leap_year(y: i32) -> bool {
            (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
        }

        fn days_of_month(m: i32) -> i32 {
            match m {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 => 28,
                _ => unreachable!(),
            }
        }

        let mut days = (1971..year)
            .map(|y| if is_leap_year(y) { 366 } else { 365 })
            .sum::<i32>();
        days += (1..month).map(days_of_month).sum::<i32>();
        if is_leap_year(year) && month > 2 {
            days += 1;
        }
        days += day - 1;
        WEEK[(days as usize + 5) % 7].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::day_of_the_week(31, 8, 2019), "Saturday");
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::day_of_the_week(18, 7, 1999), "Sunday");
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::day_of_the_week(15, 8, 1993), "Sunday");
    }
}
