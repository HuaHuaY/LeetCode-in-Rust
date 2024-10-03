pub struct Solution;

impl Solution {
    pub fn nth_person_gets_nth_seat(n: i32) -> f64 {
        if n == 1 {
            1.0
        } else {
            0.5
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::nth_person_gets_nth_seat(1), 1.0);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::nth_person_gets_nth_seat(2), 0.5);
    }
}
