pub struct Solution;

impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let mut days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        for i in 1..days.len() {
            days[i] += days[i - 1];
        }
        let parse = |str: String| {
            let mut iter = str.split('-').map(|s| s.parse::<i32>().unwrap());
            let month = iter.next().unwrap();
            let day = iter.next().unwrap();
            if month > 1 {
                day + days[month as usize - 2]
            } else {
                day
            }
        };
        let mut arrive_alice = parse(arrive_alice);
        let mut leave_alice = parse(leave_alice);
        let mut arrive_bob = parse(arrive_bob);
        let mut leave_bob = parse(leave_bob);
        if leave_bob < leave_alice {
            std::mem::swap(&mut arrive_alice, &mut arrive_bob);
            std::mem::swap(&mut leave_alice, &mut leave_bob);
        }
        if leave_alice < arrive_bob {
            0
        } else {
            leave_alice - arrive_bob.max(arrive_alice) + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_days_together(
                "08-15".to_string(),
                "08-18".to_string(),
                "08-16".to_string(),
                "08-19".to_string()
            ),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::count_days_together(
                "10-01".to_string(),
                "10-31".to_string(),
                "11-01".to_string(),
                "12-31".to_string()
            ),
            0
        );
    }
}
